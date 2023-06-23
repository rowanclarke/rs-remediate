mod cards;
mod deserialize;
mod parser;
mod serialize;

use cards::Segment;
pub use deserialize::deserialize;
pub use serialize::serialize;
use std::{collections::BTreeMap, rc::Rc};

type RemGroupCard = BTreeMap<(Rc<str>, usize, Rc<str>), Vec<Segment>>;
