use std::rc::Rc;

use crate::file::{files, open, read};

use super::{CardId, Deck, Segment, PATH};
use rkyv::{check_archived_root, de::deserializers::SharedDeserializeMap, Archived, Deserialize};

/*
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



pub fn all() -> impl Iterator<Item = Deck> {
    files().map(|file| deserialize(&file[..]))
}
*/

fn archived(bytes: &[u8]) -> &Archived<Deck> {
    check_archived_root::<Deck>(bytes).unwrap()
}

fn deserialize(bytes: &[u8]) -> Deck {
    check_archived_root::<Deck>(bytes)
        .unwrap()
        .deserialize(&mut SharedDeserializeMap::new())
        .unwrap()
}

pub fn get(path: Rc<str>, id: Archived<CardId>) -> Vec<Segment> {
    archived(read(open(&[PATH, path.as_ref()])).as_slice())
        .get(&id)
        .unwrap()
        .deserialize(&mut SharedDeserializeMap::new())
        .unwrap()
}

pub fn all() -> impl Iterator<Item = (Rc<str>, Deck)> {
    files(&[PATH]).map(|(path, file)| (path.to_str().unwrap().into(), deserialize(&read(file))))
}
