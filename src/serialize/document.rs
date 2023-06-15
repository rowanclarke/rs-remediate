use super::SerializeError;
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "serialize/document.pest"]
struct DocumentParser;

pub fn parse(input: &str) -> Result<Document, SerializeError<Rule>> {
    DocumentParser::parse(Rule::document, input)
        .map_err(|e| SerializeError::<Rule>::ParseError(e))
        .and_then(|mut pairs| {
            Document::from_pest(&mut pairs)
                .map_err(|_c| SerializeError::<Rule>::UnmatchingGrammarError)
        })
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::document))]
pub struct Document<'pest> {
    pub rems: Vec<Rem<'pest>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::rem))]
pub struct Rem<'pest> {
    pub id: Id<'pest>,
    pub content: Vec<Content<'pest>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::id))]
pub struct Id<'pest>(#[pest_ast(outer(with(as_str)))] pub &'pest str);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::content))]
pub enum Content<'pest> {
    Text(Text<'pest>),
    Closure(Closure<'pest>),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::text))]
pub struct Text<'pest>(#[pest_ast(outer(with(as_str)))] pub &'pest str);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::closure))]
pub struct Closure<'pest> {
    pub group: Group<'pest>,
    pub text: Text<'pest>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::group))]
pub struct Group<'pest>(#[pest_ast(outer(with(as_str)))] pub &'pest str);

fn as_str(span: Span) -> &str {
    span.as_str()
}
