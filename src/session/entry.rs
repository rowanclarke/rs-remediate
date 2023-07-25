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

impl<C, D: Review> Entry<C, D> {
    pub fn new(location: Rc<[C]>, id: CardId, data: D) -> Self {
        Self { location, id, data }
    }

    pub fn location(&self) -> Rc<[C]> {
        self.location.clone()
    }

    pub fn id(&self) -> CardId {
        self.id.clone()
    }

    pub fn data_mut(&mut self) -> &mut D {
        &mut self.data
    }
}

impl<C, D: Ord> Ord for Entry<C, D> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl<C, D: Ord> PartialOrd for Entry<C, D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl<C, D: Ord> PartialEq for Entry<C, D> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<C, D: Ord> Eq for Entry<C, D> {}
