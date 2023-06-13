use from_pest::{ConversionError, FromPest};
use pest::{error::Error, Parser, Span};
use pest_ast::FromPest;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "document.pest"]
struct DocumentParser;

#[derive(Debug)]
pub enum ParseError<Rule, FatalError> {
    Error(Error<Rule>),
    UnmatchingAstError(ConversionError<FatalError>),
}

pub fn parse(input: &str) -> Result<Document, ParseError<Rule, <Rem as FromPest>::FatalError>> {
    DocumentParser::parse(Rule::document, input)
        .map_err(|e| ParseError::Error(e))
        .and_then(|mut pairs| {
            Document::from_pest(&mut pairs).map_err(|c| ParseError::UnmatchingAstError(c))
        })
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::document))]
pub struct Document<'pest> {
    rems: Vec<Rem<'pest>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::rem))]
pub struct Rem<'pest> {
    id: Id<'pest>,
    content: Vec<Content<'pest>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::id))]
struct Id<'pest>(#[pest_ast(outer(with(as_str)))] &'pest str);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::content))]
enum Content<'pest> {
    Text(Text<'pest>),
    Closure(Closure<'pest>),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::text))]
struct Text<'pest>(#[pest_ast(outer(with(as_str)))] &'pest str);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::closure))]
struct Closure<'pest> {
    group: Group<'pest>,
    text: Text<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::group))]
struct Group<'pest>(#[pest_ast(outer(with(as_str)))] &'pest str);

fn as_str(span: Span) -> &str {
    span.as_str()
}
