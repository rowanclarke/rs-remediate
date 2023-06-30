use std::rc::Rc;

use super::serialize::SerializeError;
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "document/parser.pest"]
struct DocumentParser;

pub fn parse(input: &str) -> Result<Document, SerializeError<Rule>> {
    DocumentParser::parse(Rule::document, input)
        .map_err(|e| SerializeError::<Rule>::ParseError(e))
        .and_then(|mut pairs| {
            Document::from_pest(&mut pairs)
                .map_err(|_c| SerializeError::<Rule>::UnmatchingGrammarError)
        })
}

impl Document {
    pub fn rems_iter(&self) -> impl Iterator<Item = &Rem> + '_ {
        self.rems.iter()
    }
}

impl Rem {
    pub fn id(&self) -> Rc<str> {
        self.id.0.clone()
    }

    pub fn content_iter(&self) -> impl Iterator<Item = &Content> + '_ {
        self.content.iter()
    }

    pub fn children_iter(&self) -> impl Iterator<Item = &Rem> + '_ {
        self.children.iter()
    }
}

impl Content {
    pub fn text(&self) -> Rc<str> {
        match self {
            Self::Text(Text(text))
            | Self::Closure(Closure {
                location: _,
                group: _,
                text: Text(text),
            }) => text.clone(),
        }
    }

    pub fn closure(&self) -> Result<(usize, Rc<str>, Rc<str>), Rc<str>> {
        match self {
            Self::Closure(Closure {
                location: Location(location),
                group: Group(group),
                text: Text(text),
            }) => Ok((location.clone(), group.clone(), text.clone())),
            Self::Text(Text(text)) => Err(text.clone()),
        }
    }

    pub fn to_closure(location: usize, group: Rc<str>, text: Rc<str>) -> Self {
        Self::Closure(Closure {
            location: Location(location),
            group: Group(group),
            text: Text(text),
        })
    }

    pub fn to_text(text: Rc<str>) -> Self {
        Self::Text(Text(text))
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::document))]
pub struct Document {
    rems: Vec<Rem>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::rem))]
pub struct Rem {
    id: Id,
    content: Vec<Content>,
    children: Vec<Rem>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::id))]
pub struct Id(#[pest_ast(outer(with(as_str)))] Rc<str>);

#[derive(Debug, FromPest, Clone)]
#[pest_ast(rule(Rule::content))]
pub enum Content {
    Text(Text),
    Closure(Closure),
}

#[derive(Debug, FromPest, Clone)]
#[pest_ast(rule(Rule::text))]
pub struct Text(#[pest_ast(outer(with(as_str)))] Rc<str>);

#[derive(Debug, FromPest, Clone)]
#[pest_ast(rule(Rule::closure))]
pub struct Closure {
    location: Location,
    group: Group,
    text: Text,
}

#[derive(Debug, FromPest, Clone)]
#[pest_ast(rule(Rule::location))]
pub struct Location(#[pest_ast(outer(with(count)))] usize);

#[derive(Debug, FromPest, Clone)]
#[pest_ast(rule(Rule::group))]
pub struct Group(#[pest_ast(outer(with(as_str)))] Rc<str>);

fn count(span: Span) -> usize {
    span.as_str().len()
}

fn as_str(span: Span) -> Rc<str> {
    span.as_str().into()
}
