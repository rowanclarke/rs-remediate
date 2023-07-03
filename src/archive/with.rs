use std::marker::PhantomData;

pub mod binary_heap;

pub struct AsBoxedSlice<T>(PhantomData<T>);
