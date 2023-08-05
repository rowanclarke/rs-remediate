use pest::Span;
use pest_ast::FromPest;
use pest_derive::Parser;
use std::rc::Rc;

#[derive(Parser)]
#[grammar = "document/parser.pest"]
pub struct DocumentParser;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::document))]
pub struct Document {
    rems: Vec<Rem>,
}

impl From<Document> for super::Document {
    fn from(value: Document) -> Self {
        fn resolve_text(lines: Vec<Segment>) -> super::Text {
            lines
                .iter()
                .map(|line| match line {
                    Segment::Line(Line(line)) => line.as_ref(),
                    Segment::NewLine(_) => "\n",
                })
                .flat_map(|s| s.chars())
                .collect::<String>()
                .into()
        }
        fn resolve_rem((mut parents, rem): (Vec<Rc<str>>, Rem)) -> super::Rem {
            parents.push(rem.id.0.clone());
            super::Rem {
                id: rem.id.0,
                content: rem
                    .content
                    .into_iter()
                    .map(|content| match content {
                        Content::Text(Text(text)) => super::Content::Text(resolve_text(text)),
                        Content::Closure(Closure {
                            location: Location(location),
                            group: Group(group),
                            text: Text(text),
                        }) => super::Content::Closure(
                            (parents[parents.len() - location - 1].clone(), group),
                            resolve_text(text),
                        ),
                    })
                    .collect::<Vec<_>>()
                    .into(),
                children: rem
                    .children
                    .into_iter()
                    .map(|rem| (parents.clone(), rem))
                    .map(resolve_rem)
                    .collect::<Vec<_>>()
                    .into(),
            }
        }
        Self {
            rems: value
                .rems
                .into_iter()
                .map(|rem| (Vec::<Rc<str>>::new(), rem))
                .map(resolve_rem)
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::rem))]
struct Rem {
    id: Id,
    tags: Vec<Tag>,
    content: Vec<Content>,
    children: Vec<Rem>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::tag))]
struct Tag(#[pest_ast(outer(with(as_str)))] Rc<str>);

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
struct Text(Vec<Segment>);

#[derive(Debug, FromPest, Clone)]
#[pest_ast(rule(Rule::segment))]
enum Segment {
    Line(Line),
    NewLine(NewLine),
}

#[derive(Debug, FromPest, Clone)]
#[pest_ast(rule(Rule::line))]
struct Line(#[pest_ast(outer(with(as_str)))] Rc<str>);

#[derive(Debug, FromPest, Clone)]
#[pest_ast(rule(Rule::newline))]
struct NewLine(#[pest_ast(outer(with(empty)))] ());

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

fn empty(_: Span) -> () {
    ()
}
