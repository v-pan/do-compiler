use crate::Span;
use super::{SyntaxError, Parse};
use super::identifier::Identifier;

use std::io::BufReader;
use std::fs::File;

#[derive(Clone, Debug)]
pub enum Type {
    Explicit(Identifier, Span),
    Inferred,
}
impl Type {
    pub fn from(value: &str, span: Span) -> Self {
        if value == "" {
            return Type::Inferred;
        }

        let identifier = Identifier::new(&value.trim(), span);
        let span = identifier.span;

        Type::Explicit(identifier, span)
    }
}
impl Parse for Type {
    fn parse(contents: &mut BufReader<&File>, span: Span) -> Result<Self, SyntaxError> {
        todo!()
    }
}
