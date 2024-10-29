use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("Unexpected token {found}")]
#[diagnostic()]
pub struct UnexpectedToken {
    pub found: String,

    #[label = "unexpected token"]
    pub unexpected_span: SourceSpan,

    #[help]
    pub advice: Option<String>,
}
