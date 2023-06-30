use std::cmp::Ordering;

use crate::{document::CardId, schedule::Review};

#[derive(Debug)]
pub struct Entry<D> {
    id: CardId,
    data: D,
}

impl<D: Review> Entry<D> {
    pub fn new(id: CardId, data: D) -> Self {
        Self { id, data }
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
