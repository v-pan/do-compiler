use crate::token::Token;

use super::context::{ParseCtx, FunctionCtx};
use crate::token::TokenType;

pub struct Parser {}

impl Parser {
    pub fn parse(tokens: &Vec<Token>) -> Vec<Token> {
        let mut stack: Vec<ParseCtx> = vec![];
        let mut parsed: Vec<Token> = Vec::with_capacity(tokens.capacity());
        for (idx, token) in tokens.into_iter().enumerate() {
            match token.ty {
                TokenType::FunctionDecl => Parser::handle_func(token, &mut parsed, &mut stack),
                TokenType::Identifier => Parser::handle_identifier(token, &mut parsed, &mut stack),
                _ => {},
            }
        }

        parsed
    }

    fn handle_func(token: &Token, parsed: &mut Vec<Token>, stack: &mut Vec<ParseCtx>) {
        // There might have been modifiers before our declaration
        if let Some(ParseCtx::ModifierCtx(idx)) = stack.last() {
            let placeholder = parsed[*idx];

            parsed[*idx] = Token::new(placeholder.loc, TokenType::FunctionDecl);

            stack.pop(); // Modifiers handled, pop the ctx off the stack
        } else {
            parsed.push(Token::new(token.loc, token.ty));
        }

        // Declare the function context
        stack.push(FunctionCtx::new());
    }

    fn handle_identifier(token: &Token, parsed: &mut Vec<Token>, stack: &mut Vec<ParseCtx>) {
        todo!("Handle identifier interning")
    }
}
