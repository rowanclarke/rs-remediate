use std::rc::Rc;

use crate::file::{files, open, read};

use super::{Card, Deck, PATH};
use rkyv::{archived_root, de::deserializers::SharedDeserializeMap, Archived, Deserialize};

fn archived(bytes: &[u8]) -> &Archived<Deck> {
    unsafe { archived_root::<Deck>(bytes) }
}

fn deserialize(bytes: &[u8]) -> Deck {
    unsafe { archived_root::<Deck>(bytes) }
        .deserialize(&mut SharedDeserializeMap::new())
        .unwrap()
}

pub fn card(path: Rc<str>, id: Rc<str>) -> Card {
    archived(read(open(&[PATH, path.as_ref()])).as_slice())
        .get(id.as_ref())
        .unwrap()
        .deserialize(&mut SharedDeserializeMap::new())
        .unwrap()
}

pub fn decks() -> impl Iterator<Item = (Rc<str>, Deck)> {
    files(&[PATH]).map(|(path, file)| (path.to_str().unwrap().into(), deserialize(&read(file))))
}
