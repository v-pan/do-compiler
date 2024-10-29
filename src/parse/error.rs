use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::token::Token;

#[derive(Error, Diagnostic, Debug)]
#[error("Unexpected token. Expected {expected:?} found {found:?}.")]
#[diagnostic()]
pub struct UnexpectedToken<'a> {
    expected: Token<'a>,
    found: Token<'a>,

    #[label = "unexpected token"]
    unexpected_span: SourceSpan,

    #[help]
    advice: String,
}
