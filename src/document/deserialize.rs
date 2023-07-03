use std::rc::Rc;

use crate::file::{files, open, read};

use super::{CardId, Deck, Segment, PATH};
use rkyv::{check_archived_root, de::deserializers::SharedDeserializeMap, Archived, Deserialize};

fn archived(bytes: &[u8]) -> &Archived<Deck> {
    check_archived_root::<Deck>(bytes).unwrap()
}

fn deserialize(bytes: &[u8]) -> Deck {
    check_archived_root::<Deck>(bytes)
        .unwrap()
        .deserialize(&mut SharedDeserializeMap::new())
        .unwrap()
}

pub fn get(path: Rc<str>, id: &Archived<CardId>) -> Vec<Segment> {
    archived(read(open(&[PATH, path.as_ref()])).as_slice())
        .get(id)
        .unwrap()
        .deserialize(&mut SharedDeserializeMap::new())
        .unwrap()
}

pub fn all() -> impl Iterator<Item = (Rc<str>, Deck)> {
    files(&[PATH]).map(|(path, file)| (path.to_str().unwrap().into(), deserialize(&read(file))))
}
