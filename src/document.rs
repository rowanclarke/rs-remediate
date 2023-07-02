mod cards;
pub mod deserialize;
mod parser;
mod serialize;

pub use cards::Segment;
use rkyv::{Archive, Deserialize, Serialize};
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
