use miette::{Diagnostic, LabeledSpan, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[diagnostic()]
#[error("Unexpected token {found:?}")]
pub struct UnexpectedToken {
    pub found: String,

    pub unexpected_span: LabeledSpan,
}

impl UnexpectedToken {
    fn expected(self) {}
}
