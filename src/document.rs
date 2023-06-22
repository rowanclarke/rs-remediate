mod cards;
mod parser;
pub mod serialize;

use cards::Segment;
use std::{collections::BTreeMap, rc::Rc};

type RemGroupCard = BTreeMap<(Rc<str>, usize, Rc<str>), Vec<Segment>>;
