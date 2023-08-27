use crate::Span;
use super::{SyntaxError, Parse};
use super::function::Function;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::from_utf8;

// Not an AST node
pub enum Keyword {
    Function(Function),
}
impl Parse for Keyword {
    fn parse(reader: &mut BufReader<&File>, span: Span) -> Result<Self, SyntaxError> {
        let mut buf = Vec::<u8>::new();
        reader.read_until(b' ', &mut buf).unwrap();
        let word = from_utf8(&buf).unwrap();

        match word.trim_end() {
            "fun" => { Ok(Keyword::Function(Function::parse(reader, span)?)) }
            _ => { panic!("Expected keyword") }
        }
    }
}
