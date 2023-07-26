use super::AsVec;
use rkyv::{
    ser::{ScratchSpace, Serializer},
    with::{ArchiveWith, DeserializeWith, SerializeWith},
    Archive, Archived, Deserialize, DeserializeUnsized, Fallible, Resolver, Serialize,
};
use std::collections::BinaryHeap;

impl<T: Archive> ArchiveWith<BinaryHeap<T>> for AsVec {
    type Archived = Archived<Vec<T>>;
    type Resolver = Resolver<Vec<T>>;

    unsafe fn resolve_with(
        field: &BinaryHeap<T>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        let ptr = field as *const BinaryHeap<T> as *const Vec<T>;
        Self::Archived::resolve_from_slice(&*ptr, pos, resolver, out)
    }
}

impl<T: Archive + Serialize<S>, S: ScratchSpace + Serializer + ?Sized>
    SerializeWith<BinaryHeap<T>, S> for AsVec
{
    fn serialize_with(
        field: &BinaryHeap<T>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, <S as Fallible>::Error> {
        let ptr = field as *const BinaryHeap<T> as *const Vec<T>;
        Self::Archived::serialize_from_slice(unsafe { &*ptr }, serializer)
    }
}

impl<T: Archive + Ord, D: Fallible + ?Sized> DeserializeWith<Archived<Vec<T>>, BinaryHeap<T>, D>
    for AsVec
where
    [Archived<T>]: DeserializeUnsized<[T], D>,
{
    fn deserialize_with(
        field: &Archived<Vec<T>>,
        deserializer: &mut D,
    ) -> Result<BinaryHeap<T>, <D as Fallible>::Error> {
        field.as_slice();
        let data = field.deserialize(deserializer)?;
        let mut heap = BinaryHeap::<T>::new();
        let ptr = &mut heap as *mut BinaryHeap<T> as *mut Vec<T>;
        unsafe {
            ptr.write(data);
        }
        Ok(heap)
    }
}
