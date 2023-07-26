use std::collections::BinaryHeap;

use crate::archive::{with::AsVec, Cast};
use rkyv::{
    de::deserializers::SharedDeserializeMap,
    ser::{serializers::AllocSerializer, Serializer},
    with::With,
    Archived, Deserialize,
};

#[test]
fn binary_heap() {
    let mut serializer = AllocSerializer::<1024>::default();
    let heap = BinaryHeap::<i32>::from(vec![1, 11, 2, 7, 4, 5, 8, 3, 16, 2]);
    serializer
        .serialize_value(With::<_, AsVec>::cast(&heap))
        .unwrap();
    let bytes = serializer.into_serializer().into_inner();
    let as_vec = bytes.as_slice().cast() as &Archived<Vec<i32>>;
    let mut deserializer = SharedDeserializeMap::default();
    let with_heap: With<BinaryHeap<i32>, AsVec> = as_vec.deserialize(&mut deserializer).unwrap();
    assert_eq!(with_heap.into_inner().into_vec(), heap.into_vec());
}
