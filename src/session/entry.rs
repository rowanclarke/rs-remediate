use crate::{
    document::{parser::Group, CardId},
    schedule::Review,
};
use rkyv::{Archive, Deserialize, Serialize};
use std::{cmp::Ordering, rc::Rc};

#[derive(Debug, Archive, Serialize, Deserialize)]
#[archive(check_bytes)]
pub struct Entry<D> {
    path: Rc<str>,
    id: CardId,
    data: D,
}

impl<D: Review> Entry<D> {
    pub fn new(path: Rc<str>, id: CardId, data: D) -> Self {
        Self { path, id, data }
    }

    pub fn into_components(self) -> (Rc<str>, CardId, D) {
        (self.path, self.id, self.data)
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
