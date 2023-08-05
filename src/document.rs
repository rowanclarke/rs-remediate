mod ast;

use ast::DocumentParser;
pub use ast::Rule;
use from_pest::FromPest;
use pest::{error::Error, Parser};
use rkyv::{Archive, Deserialize, Serialize};
use std::{rc::Rc, str};

#[derive(Debug)]
pub struct Document {
    rems: Rc<[Rem]>,
}

#[derive(Debug)]
pub struct Rem {
    id: Rc<str>,
    content: Rc<[Content]>,
    children: Rc<[Rem]>,
}

#[derive(Debug, Clone, Archive, Serialize, Deserialize, Eq, PartialEq)]
#[archive_attr(derive(Debug))]
pub enum Content {
    Text(Text),
    Closure(Group, Text),
}

pub type Group = (Rc<str>, Rc<str>);

pub type Text = Rc<str>;

#[derive(Debug, Clone, Archive, Serialize, Deserialize, Eq, PartialEq)]
#[archive_attr(derive(Debug))]
pub enum Segment {
    NewLine,
    Line(Box<str>),
}

impl Document {
    pub fn parse(input: &str) -> Result<Self, Error<Rule>> {
        let mut pairs = DocumentParser::parse(Rule::document, input)?;
        Ok(ast::Document::from_pest(&mut pairs).unwrap().into())
    }

    pub fn rems(&self) -> Rc<[Rem]> {
        self.rems.clone()
    }
}

impl Rem {
    pub fn id(&self) -> Rc<str> {
        self.id.clone()
    }

    pub fn content(&self) -> Rc<[Content]> {
        self.content.clone()
    }

    pub fn children(&self) -> Rc<[Rem]> {
        self.children.clone()
    }
}

impl Content {
    pub fn group(&self) -> Option<Group> {
        match self {
            Self::Closure(group, _) => Some(group.clone()),
            _ => None,
        }
    }
}
