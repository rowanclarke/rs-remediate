use crate::{deck::CardId, schedule::Review};
use rkyv::{Archive, Deserialize, Serialize};
use std::{cmp::Ordering, rc::Rc};

#[derive(Debug, Archive, Serialize, Deserialize)]
#[archive(check_bytes)]
pub struct Entry<C, D> {
    location: Rc<[C]>,
    id: CardId,
    data: D,
}

impl<L, D: Review> Entry<L, D> {
    pub fn new(location: Rc<[L]>, id: CardId, data: D) -> Self {
        Self { location, id, data }
    }

    pub fn location(&self) -> Rc<[L]> {
        self.location.clone()
    }

    pub fn id(&self) -> CardId {
        self.id.clone()
    }

    pub fn data_mut(&mut self) -> &mut D {
        &mut self.data
    }
}

impl<L, D: Ord> Ord for Entry<L, D> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl<L, D: Ord> PartialOrd for Entry<L, D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl<L, D: Ord> PartialEq for Entry<L, D> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<L, D: Ord> Eq for Entry<L, D> {}
