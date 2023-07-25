use rkyv::{Archive, Archived, Deserialize, Fallible, Serialize};
use std::cmp::{self, Ordering};

#[derive(Debug)]
pub struct Reverse<T>(cmp::Reverse<T>);

impl<T> Reverse<T> {
    pub fn new(data: T) -> Self {
        Self(cmp::Reverse(data))
    }

    pub fn get_ref(&self) -> &T {
        &(self.0).0
    }

    pub fn get_mut(&mut self) -> &T {
        &mut (self.0).0
    }

    pub fn get(self) -> T {
        (self.0).0
    }
}

impl<T: Ord> Ord for Reverse<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: PartialOrd> PartialOrd for Reverse<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: PartialEq> PartialEq for Reverse<T> {
    fn eq(&self, other: &Self) -> bool {
        &self.0 == &other.0
    }
}

impl<T: Eq> Eq for Reverse<T> {}

impl<T: Archive> Archive for Reverse<T> {
    type Archived = T::Archived;
    type Resolver = T::Resolver;

    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.get_ref().resolve(pos, resolver, out)
    }
}

impl<S: Fallible + ?Sized, T: Serialize<S>> Serialize<S> for Reverse<T> {
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, <S as Fallible>::Error> {
        self.get_ref().serialize(serializer)
    }
}

impl<D: Fallible + ?Sized, T: Archive> Deserialize<Reverse<T>, D> for Archived<T>
where
    Archived<T>: Deserialize<T, D>,
{
    fn deserialize(&self, deserializer: &mut D) -> Result<Reverse<T>, <D as Fallible>::Error> {
        Ok(Reverse(cmp::Reverse(self.deserialize(deserializer)?)))
    }
}
