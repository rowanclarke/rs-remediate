use pest::{error::Error, RuleType};
use std::{
    fmt::{Debug, Display},
    io,
};

#[derive(Debug)]
pub enum SerializeError<Rule> {
    ParseError(Error<Rule>),
    FileError(io::Error),
    UnmatchingGrammarError,
    SerializeError,
    EnvironmentError,
}

impl<Rule: RuleType> Display for SerializeError<Rule> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(e) => write!(f, "{}", e),
            Self::FileError(e) => write!(f, "{}", e),
            Self::UnmatchingGrammarError => write!(f, "Syntax and grammar do not match."),
            Self::SerializeError => write!(f, "Cannot serialize object."),
            Self::EnvironmentError => {
                write!(f, "'REMEDY_DIR' environment variable cannot be found.")
            }
        }
    }
}
