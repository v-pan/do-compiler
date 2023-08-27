mod identifier;
mod keyword;
mod function;
mod list;
mod r#type;

pub use identifier::Identifier as Identifier;
pub use keyword::Keyword as Keyword;

use crate::span::Span;

use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub struct SyntaxError {
    pub description: String,
    pub valid_until: Span
}
impl SyntaxError {
    pub fn new(description: &str, valid_until: Span) -> Self {
        SyntaxError { description: description.to_string(), valid_until }
    }
}
impl std::error::Error for SyntaxError {
    fn description(&self) -> &str {
        self.description.as_ref()
    }
}
impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse token.")
    }
}

pub trait Parse: Sized {
    fn parse(reader: &mut BufReader<&File>, span: Span) -> Result<Self, SyntaxError>;
}
