use crate::token::{TokenType, Token};

use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub struct UnexpectedToken { token: Token, expected_type: TokenType }
impl UnexpectedToken {
    pub fn new(token: Token, expected_type: TokenType) -> ParseError {
        ParseError::UnexpectedToken(UnexpectedToken { token, expected_type })
    }
}
impl Display for UnexpectedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected {:?} at byte {}, found {:?}", self.expected_type, self.token.loc, self.token.ty)
    }
}

#[derive(Debug)]
pub enum ParseError {
    Unknown,
    UnexpectedToken(UnexpectedToken),
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Unknown => write!(f, "Unknown error"),
            ParseError::UnexpectedToken(err) => write!(f, "{err}"),
        }
    }
}
impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ParseError::Unknown => None,
            ParseError::UnexpectedToken(..) => None,
        }
    }
}
// impl From<PackingError> for ParseError {
//     fn from(value: PackingError) -> Self {
//         ParseError::Packing(value)
//     }
// }