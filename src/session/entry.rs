use crate::{document::CardId, schedule::Review};
use rkyv::{Archive, Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Archive, Serialize, Deserialize)]
#[archive(check_bytes)]
pub struct Entry<D> {
    path: String,
    id: CardId,
    data: D,
}

impl<D: Review> Entry<D> {
    pub fn new(path: String, id: CardId, data: D) -> Self {
        Self { path, id, data }
    }

    pub fn unwrap(self) -> (CardId, D) {
        (self.id, self.data)
    }

    pub fn review(&mut self, score: <D as Review>::Score) {
        self.data.review(score)
    }

    pub fn id(&self) -> &CardId {
        &self.id
    }
}

impl<D: Ord> Ord for Entry<D> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl<D: Ord> PartialOrd for Entry<D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl<D: Ord> PartialEq for Entry<D> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<D: Ord> Eq for Entry<D> {}
