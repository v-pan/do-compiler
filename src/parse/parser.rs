use crate::token::{Token, TokenType, Associativity};

use super::node::ParseNode;

pub struct Parser {}

impl Parser {
    // Returns a clone of the token buffer such that a linear walk = post order traversal
    pub fn parse(tokens: &Vec<Token>) -> Vec<ParseNode> {
        let mut parsed: Vec<ParseNode> = Vec::with_capacity(tokens.len());

        let mut stack: Vec<ParseNode> = vec![];

        for (idx, token) in tokens.iter().enumerate() {
            match token.ty {
                TokenType::OpenParen => {
                    if let Some(last_token_idx) = parsed.pop() {
                        // Check for an identifier preceeding the (
                        let last_token = tokens[last_token_idx];
                        if let TokenType::Unknown = last_token.ty {
                            // If one exists, assume its a function call and treat as a unary operator
                            stack.push(last_token_idx);
                        } else {
                            parsed.push(last_token_idx);
                        }
                    }
                    stack.push(idx);
                    parsed.push(idx);
                }
                TokenType::CloseParen => {
                    while let Some(last) = stack.pop() {
                        let last_type = tokens[last].ty;
                        if let TokenType::OpenParen = last_type {
                            parsed.push(idx);
                            break;
                        }

                        parsed.push(last);
                    }
                }
                TokenType::SemiColon => {
                    while let Some(last) = stack.pop() {
                        parsed.push(last);
                    }
                    parsed.push(idx);
                },
                _ => if let Some((precedence, associativity)) = token.ty.try_operator() {
                    loop {
                        match stack.pop() {
                            Some(last_idx) => {
                                let last_type = tokens[last_idx].ty;

                                if let Some((last_precedence, _)) = last_type.try_operator() {
                                    if last_precedence >= precedence {
                                        parsed.push(last_idx);
                                    } else {
                                        stack.push(last_idx); // Keep operator in stack if not higher precedence
                                        stack.push(idx);
                                        break;
                                    }
                                } else {
                                    stack.push(last_idx);
                                    stack.push(idx);
                                    break;
                                }
                            }
                            None => {
                                stack.push(idx);
                                break;
                            }
                        }
                        
                    }
                } else {
                    let token_type = tokens[idx].ty;
                    parsed.push(idx);
                }
            }
        }

        for token_idx in stack.into_iter().rev() {
            let token = tokens[token_idx];
            if let TokenType::OpenParen = token.ty {
                panic!("Got unexpected (");
            }

            parsed.push(token_idx);
        }
        
        parsed
    }
}