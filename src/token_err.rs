use std::{fmt::Display, error::Error};

use packed_struct::PackingError;

#[derive(Debug)]
pub enum TokenizationError {
    NoMatch,
    Packing(PackingError)
}
impl Display for TokenizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizationError::NoMatch => write!(f, "Found no matching token"),
            TokenizationError::Packing(err) => write!(f, "Could not pack token")
        }
    }
}
impl Error for TokenizationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            TokenizationError::NoMatch => None,
            TokenizationError::Packing(ref err) => Some(err)
        }
    }
}
impl From<PackingError> for TokenizationError {
    fn from(value: PackingError) -> Self {
        TokenizationError::Packing(value)
    }
}