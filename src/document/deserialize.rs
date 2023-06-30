use super::{Deck, PATH};
use crate::REMEDY_DIR;
use rkyv::{check_archived_root, de::deserializers::SharedDeserializeMap, Deserialize, Infallible};
use std::{
    env,
    fs::{read_dir, File},
    io,
    io::prelude::Read,
    path::Path,
};

fn files() -> impl Iterator<Item = Vec<u8>> {
    let rem_dir = &env::var(REMEDY_DIR).unwrap();
    let dir = Path::new(&rem_dir).join(PATH);
    read_dir(dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter_map(|entry| File::open(entry.path()).ok())
        .map(|mut file| {
            let mut buf = vec![];
            file.read_to_end(&mut buf).unwrap();
            buf
        })
}

fn deserialize(bytes: &[u8]) -> Deck {
    check_archived_root::<Deck>(bytes)
        .unwrap()
        .deserialize(&mut SharedDeserializeMap::new())
        .unwrap()
}

pub fn all() -> impl Iterator<Item = Deck> {
    files().map(|file| deserialize(&file[..]))
}
