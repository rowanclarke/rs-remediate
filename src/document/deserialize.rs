use super::RemGroupCard;
use crate::REMEDY_DIR;
use rkyv::{check_archived_root, de::deserializers::SharedDeserializeMap, Deserialize};
use std::{
    env,
    fs::{read_dir, File},
    io::prelude::Read,
};

pub fn deserialize() -> RemGroupCard {
    let mut result = RemGroupCard::new();
    let dir = &env::var(REMEDY_DIR).unwrap();
    for entry in read_dir(dir).unwrap().filter_map(Result::ok) {
        let mut file = File::open(entry.path()).unwrap();
        let mut buf = vec![];
        file.read_to_end(&mut buf).unwrap();
        result.append(
            &mut check_archived_root::<RemGroupCard>(&buf[..])
                .unwrap()
                .deserialize(&mut SharedDeserializeMap::new())
                .unwrap(),
        );
    }
    result
}
