use crate::token::Token;

pub struct Parser {}

impl Parser {
    fn new() -> Self {
        Self {}
    }

    fn parse(tokens: &Vec<Token>) -> Vec<Token> {
        tokens.clone()
    }
}