mod archive;
pub mod deck;
pub mod document;
pub mod schedule;
pub mod session;
pub mod tags;
#[macro_use]
pub mod workspace;

#[cfg(test)]
mod tests;

use std::rc::Rc;
use workspace::{AsComponents, Component, IntoComponents, Root};

root!(pub type RemedyRoot = [".rem"]);
