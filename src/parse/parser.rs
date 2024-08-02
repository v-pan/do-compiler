use std::io::BufReader;

use string_interner::StringInterner;

use crate::{lexer::AsciiLexer, token::Token};

use super::context::{FunctionCtx, ParseCtx};
use crate::token::TokenType;

pub struct Parser<B: string_interner::backend::Backend> {
    pub idents: StringInterner<B>,
}

impl<B: string_interner::backend::Backend> From<AsciiLexer<B>> for Parser<B> {
    fn from(value: AsciiLexer<B>) -> Self {
        Parser::new(StringInterner::new())
    }
}

impl<B: string_interner::backend::Backend> Parser<B> {
    pub fn new(idents: StringInterner<B>) -> Self {
        Parser { idents }
    }

    pub fn parse<T: std::io::Read + std::io::Seek>(
        &mut self,
        tokens: &Vec<Token>,
        reader: &mut BufReader<T>,
    ) -> Vec<Token> {
        let mut stack: Vec<Token> = vec![];
        let mut parsed: Vec<Token> = Vec::with_capacity(tokens.capacity());

        for (idx, token) in tokens.iter().enumerate() {
            let params = ParserParams {
                idx,
                tokens,
                stack: &mut stack,
                parsed: &mut parsed,
            };

            match token.ty {
                TokenType::VariableDecl => self.handle_variable_decl(params),
                TokenType::Identifier => self.handle_identifier(params, reader),
                TokenType::Equals => self.handle_equals(params),

                TokenType::Space => {}
                _ => {
                    todo!("{:?}", &token.ty)
                }
            }

            println!("Parsed, {:?}", parsed);
        }

        parsed
    }

    fn handle_variable_decl(&self, params: ParserParams<'_, B>) {
        let ParserParams {
            idx,
            tokens,
            stack,
            parsed,
        } = params;

        // Shift to identifier
        stack.push(ParseCtx::VariableCtx);
        parsed.push(tokens[idx]);
    }

    fn handle_identifier<T: std::io::Read + std::io::Seek>(
        &mut self,
        params: ParserParams<'_, B>,
        reader: &mut BufReader<T>,
    ) {
        let ParserParams {
            idx,
            tokens,
            stack,
            parsed,
        } = params;

        let ctx = unsafe { stack.last().unwrap_unchecked() };

        match ctx {
            ParseCtx::VariableCtx => {
                let token = tokens[idx];
                let symbol = self.idents.get_or_intern(token.get_string(tokens, reader));

                // Shift to assignment operator
                stack.push(ParseCtx::IdentifierCtx(symbol));
                parsed.push(tokens[idx]);
            }
            ParseCtx::PlusCtx => {
                let token = tokens[idx];
                let symbol = self.idents.get_or_intern(token.get_string(tokens, reader));

                // Reduce into post-order
                stack.pop();
                stack.push(ParseCtx::IdentifierCtx(symbol));
                parsed.push(tokens[idx]);
            }
            _ => {
                todo!("Handle all ident contexts")
            }
        }
    }

    fn handle_equals(&self, params: ParserParams<'_, B>) {
        let ParserParams {
            idx,
            tokens,
            stack,
            parsed,
        } = params;

        // Shift to expression
        stack.push(ParseCtx::EqualsCtx);
        parsed.push(tokens[idx]);
    }

    fn handle_plus(&self, params: ParserParams<'_, B>) {
        let ParserParams {
            idx,
            tokens,
            stack,
            parsed,
        } = params;

        // Shift to expression
        stack.push(ParseCtx::PlusCtx);
        parsed.push(tokens[idx]);
    }

    fn handle_func(&self, params: ParserParams<'_, B>) {
        let ParserParams {
            idx,
            tokens,
            stack,
            parsed,
        } = params;

        let token = tokens[idx];

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
}

struct ParserParams<'a, B: string_interner::backend::Backend> {
    idx: usize,
    tokens: &'a [Token],
    stack: &'a mut Vec<ParseCtx<B>>,
    parsed: &'a mut Vec<Token>,
}
