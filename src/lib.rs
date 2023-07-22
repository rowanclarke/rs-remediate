#![feature(return_position_impl_trait_in_trait, extend_one, binary_heap_as_slice)]
mod archive;
pub mod deck;
pub mod schedule;
pub mod session;
#[macro_use]
pub mod workspace;

const DIR: &str = ".rem";
