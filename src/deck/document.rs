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
    Text(Rc<str>),
    Closure(Group, Rc<str>),
}

pub type Group = (Rc<str>, Rc<str>);

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

mod ast {
    use pest::Span;
    use pest_ast::FromPest;
    use pest_derive::Parser;
    use std::rc::Rc;

    #[derive(Parser)]
    #[grammar = "deck/document.pest"]
    pub struct DocumentParser;

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::document))]
    pub struct Document {
        rems: Vec<Rem>,
    }

    impl From<Document> for super::Document {
        fn from(value: Document) -> Self {
            fn resolve((mut parents, rem): (Vec<Rc<str>>, Rem)) -> super::Rem {
                parents.push(rem.id.0.clone());
                super::Rem {
                    id: rem.id.0,
                    content: rem
                        .content
                        .into_iter()
                        .map(|content| match content {
                            Content::Text(Text(text)) => super::Content::Text(text),
                            Content::Closure(Closure {
                                location: Location(location),
                                group: Group(group),
                                text: Text(text),
                            }) => super::Content::Closure(
                                (parents[parents.len() - location - 1].clone(), group),
                                text,
                            ),
                        })
                        .collect::<Vec<_>>()
                        .into(),
                    children: rem
                        .children
                        .into_iter()
                        .map(|rem| (parents.clone(), rem))
                        .map(resolve)
                        .collect::<Vec<_>>()
                        .into(),
                }
            }
            Self {
                rems: value
                    .rems
                    .into_iter()
                    .map(|rem| (Vec::<Rc<str>>::new(), rem))
                    .map(resolve)
                    .collect::<Vec<_>>()
                    .into(),
            }
        }
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::rem))]
    struct Rem {
        id: Id,
        content: Vec<Content>,
        children: Vec<Rem>,
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::id))]
    struct Id(#[pest_ast(outer(with(as_str)))] Rc<str>);

    #[derive(Debug, FromPest, Clone)]
    #[pest_ast(rule(Rule::content))]
    enum Content {
        Text(Text),
        Closure(Closure),
    }

    #[derive(Debug, FromPest, Clone)]
    #[pest_ast(rule(Rule::text))]
    struct Text(#[pest_ast(outer(with(as_str)))] Rc<str>);

    #[derive(Debug, FromPest, Clone)]
    #[pest_ast(rule(Rule::closure))]
    struct Closure {
        location: Location,
        group: Group,
        text: Text,
    }

    #[derive(Debug, FromPest, Clone)]
    #[pest_ast(rule(Rule::location))]
    struct Location(#[pest_ast(outer(with(count)))] usize);

    #[derive(Debug, FromPest, Clone)]
    #[pest_ast(rule(Rule::group))]
    struct Group(#[pest_ast(outer(with(as_str)))] Rc<str>);

    fn count(span: Span) -> usize {
        span.as_str().len()
    }

    fn as_str(span: Span) -> Rc<str> {
        span.as_str().into()
    }
}
