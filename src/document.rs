mod cards;
pub mod deserialize;
mod parser;
mod serialize;

pub use cards::{DisplayCard, Segment};
use rkyv::{
    archived_root, ser::serializers::AllocSerializer, ser::Serializer, AlignedVec, Archive,
    Archived, Deserialize, Serialize,
};
pub use serialize::serialize;
use std::{collections::BTreeMap, rc::Rc};

const PATH: &str = "documents";

#[derive(Clone, Ord, Eq, PartialEq, PartialOrd, Archive, Serialize, Debug, Deserialize)]
#[archive(check_bytes)]
#[archive_attr(derive(Ord, PartialEq, Eq, PartialOrd))]
pub struct CardId {
    id: Rc<str>,
    location: usize,
    group: Rc<str>,
}

pub type Deck = BTreeMap<CardId, Vec<Segment>>;

type CardIdSerializer = AllocSerializer<1024>;

impl CardId {
    pub fn archived(&self) -> AlignedVec {
        let mut serializer = CardIdSerializer::default();
        serializer.serialize_value(self).unwrap();
        serializer.into_serializer().into_inner()
    }
}
