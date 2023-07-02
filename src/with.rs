use std::{collections::BinaryHeap, marker::PhantomData};

use rkyv::{
    ser::{ScratchSpace, Serializer},
    with::{ArchiveWith, DeserializeWith, SerializeWith},
    Archive, Archived, Deserialize, DeserializeUnsized, Fallible, Resolver, Serialize,
};

pub struct AsBoxedSlice<T>(PhantomData<T>);

impl<T: Archive> ArchiveWith<BinaryHeap<T>> for AsBoxedSlice<T> {
    type Archived = Archived<Box<[T]>>;
    type Resolver = Resolver<Box<[T]>>;

    unsafe fn resolve_with(
        field: &BinaryHeap<T>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        Self::Archived::resolve_from_ref(field.as_slice(), pos, resolver, out)
    }
}

impl<T: Archive + Serialize<S>, S: ScratchSpace + Serializer + ?Sized>
    SerializeWith<BinaryHeap<T>, S> for AsBoxedSlice<T>
{
    fn serialize_with(
        field: &BinaryHeap<T>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, <S as Fallible>::Error> {
        Self::Archived::serialize_from_ref(field.as_slice(), serializer)
    }
}

impl<T: Archive + Ord, D: Fallible + ?Sized> DeserializeWith<Archived<Box<[T]>>, BinaryHeap<T>, D>
    for AsBoxedSlice<T>
where
    [Archived<T>]: DeserializeUnsized<[T], D>,
{
    fn deserialize_with(
        field: &Archived<Box<[T]>>,
        deserializer: &mut D,
    ) -> Result<BinaryHeap<T>, <D as Fallible>::Error> {
        let vec: Box<[T]> = field.deserialize(deserializer)?;
        Ok(vec.into_vec().into())
    }
}
