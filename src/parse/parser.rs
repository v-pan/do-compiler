// use std::io::BufReader;
//
// use string_interner::{backend::Backend, StringInterner};
//
// use crate::{lexer::AsciiLexer, token::Token};
//
// use super::context::ParseCtx;
// use crate::token::TokenType;
//
// pub struct Parser<B: string_interner::backend::Backend> {
//     pub idents: StringInterner<B>,
// }
//
// impl<B: string_interner::backend::Backend> From<AsciiLexer<B>> for Parser<B> {
//     fn from(value: AsciiLexer<B>) -> Self {
//         Parser::new(StringInterner::new())
//     }
// }
//
// impl<B: string_interner::backend::Backend> Parser<B> {
//     pub fn new(idents: StringInterner<B>) -> Self {
//         Parser { idents }
//     }
//
//     pub fn parse<T: std::io::Read + std::io::Seek>(
//         &mut self,
//         tokens: &Vec<Token<B>>,
//         reader: &mut BufReader<T>,
//     ) -> Vec<Token<B>> {
//         let mut stack: Vec<Token<B>> = vec![];
//         let mut parsed: Vec<Token<B>> = Vec::with_capacity(tokens.capacity());
//
//         for (idx, token) in tokens.iter().enumerate() {
//             let params = ParserParams {
//                 idx,
//                 tokens,
//                 stack: &mut stack,
//                 parsed: &mut parsed,
//             };
//
//             match token.ty {
//                 TokenType::VariableDecl => self.handle_variable_decl(params),
//                 TokenType::Identifier => self.handle_identifier(params, reader),
//                 TokenType::Equals => self.handle_equals(params),
//
//                 TokenType::Space => {}
//                 _ => {
//                     todo!("{:?}", &token.ty)
//                 }
//             }
//
//             println!("Parsed, {:?}", parsed);
//         }
//
//         parsed
//     }
//
//     fn handle_variable_decl(&self, params: ParserParams<'_, B>) {
//         let ParserParams {
//             idx,
//             tokens,
//             stack,
//             parsed,
//         } = params;
//
//         // TODO: Check for modifier tokens
//
//         // Shift to identifier
//         let token = tokens[idx].clone();
//         stack.push(token.clone());
//         parsed.push(token);
//     }
//
//     fn handle_identifier<T: std::io::Read + std::io::Seek>(
//         &mut self,
//         params: ParserParams<'_, B>,
//         reader: &mut BufReader<T>,
//     ) {
//         let ParserParams {
//             idx,
//             tokens,
//             stack,
//             parsed,
//         } = params;
//
//         let ctx = unsafe { stack.last().unwrap_unchecked() };
//
//         match ctx.ty {
//             TokenType::VariableDecl => {
//                 let token = tokens[idx].clone();
//                 // let symbol = self.idents.get_or_intern(token.get_string(tokens, reader));
//
//                 // Shift to assignment operator
//                 parsed.push(token);
//             }
//             TokenType::Equals => {
//                 let token = tokens[idx].clone();
//                 parsed.push(token);
//             }
//             _ => {
//                 todo!("Handle all ident contexts")
//             }
//         }
//     }
//
//     fn handle_equals(&self, params: ParserParams<'_, B>) {
//         let ParserParams {
//             idx,
//             tokens,
//             stack,
//             parsed,
//         } = params;
//
//         // Shift to expression
//         let token = tokens[idx].clone();
//         stack.push(token.clone());
//         parsed.push(token);
//     }
//
//     fn handle_plus(&self, params: ParserParams<'_, B>) {
//         let ParserParams {
//             idx,
//             tokens,
//             stack,
//             parsed,
//         } = params;
//
//         // Shift to expression
//         let token = tokens[idx].clone();
//         stack.push(token.clone());
//         parsed.push(token);
//     }
//
//     fn handle_func(&self, params: ParserParams<'_, B>) {
//         let ParserParams {
//             idx,
//             tokens,
//             stack,
//             parsed,
//         } = params;
//
//         let token = tokens[idx].clone();
//
//         // There might have been modifiers before our declaration
//         // if let Some(TokenType::PublicModifier) = stack.last() {
//         //     let placeholder = parsed[*idx];
//
//         //     parsed[*idx] = Token::new(placeholder.loc, TokenType::FunctionDecl);
//
//         //     stack.pop(); // Modifiers handled, pop the ctx off the stack
//         // } else {
//         parsed.push(Token::new(token.loc, token.ty));
//         // }
//
//         // Declare the function context
//     }
// }
//
// struct ParserParams<'a, B: Backend> {
//     idx: usize,
//     tokens: &'a [Token<B>],
//     stack: &'a mut Vec<Token<B>>,
//     parsed: &'a mut Vec<Token<B>>,
// }

use crate::token::Token;

pub struct Parser<'a> {
    index: usize,
    parsed: Vec<Token<'a>>,
}

// macro_rules! is_type {
//     ($token:ident, $variant:path) => {
//         match $token.ty {
//             $variant { .. } => true,
//             _ => false,
//         }
//     };
// }

impl<'a> Parser<'a> {
    pub fn new(index: usize, parsed: Vec<Token<'a>>) -> Self {
        Parser { index, parsed }
    }

    fn peek(&self, tokens: &'a [Token<'a>]) -> Token<'a> {
        let mut index = self.index;

        loop {
            let next_token = tokens[index];
            index += 1;

            if let Token::Space | Token::Newline = next_token {
                continue;
            } else {
                return next_token;
            }
        }
    }

    fn next(&mut self, tokens: &'a [Token<'a>]) -> Token<'a> {
        loop {
            let next_token = tokens[self.index];
            self.index += 1;

            if let Token::Space | Token::Newline = next_token {
                continue;
            } else {
                return next_token;
            }
        }
    }

    pub fn parse(mut self, tokens: &'a [Token<'a>]) -> Vec<Token<'_>> {
        let mut stack: Vec<Token<'_>> = vec![];
        let mut parsed: Vec<Token<'_>> = Vec::with_capacity(tokens.len());

        let token = self.peek(tokens);

        match token {
            Token::Identifier(_) => {
                self.parse_expression(tokens, &mut stack);
            }
            _ => {}
        }

        self.parsed
    }

    fn parse_expression(&mut self, tokens: &'a [Token<'a>], stack: &mut Vec<Token<'a>>) {
        let lhs = self.next(tokens);

        stack.push(lhs);

        loop {
            let operator = self.next(tokens);

            stack.push(operator);

            match operator {
                Token::Plus(_) => self.parse_binary_operator(tokens, stack),
                Token::SemiColon(_) => {
                    break;
                }
                _ => {
                    todo!("Error handling {:?}", operator);
                }
            }
        }

        self.parsed.extend(stack.iter());
        stack.clear();
    }

    fn parse_binary_operator(&mut self, tokens: &'a [Token<'a>], stack: &mut Vec<Token<'a>>) {
        let op = stack.pop().unwrap();

        let rhs = self.next(tokens);

        stack.push(rhs);
        stack.push(op);
    }
}

impl<'a> Default for Parser<'a> {
    fn default() -> Self {
        Self::new(0, vec![])
    }
}

fn precedence() {}
