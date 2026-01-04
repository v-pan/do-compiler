use log::trace;
use miette::{miette, Context, LabeledSpan};

use crate::{
    parsed_to_str,
    token::{Inner, Token},
};

use super::error::UnexpectedToken;

pub struct Parser<'t> {
    index: usize,
    tokens: &'t [Token<'t>],
    stack: Vec<Token<'t>>,
    parsed: Vec<Token<'t>>,
}

impl<'t> Parser<'t> {
    pub fn new(index: usize, tokens: &'t [Token<'t>]) -> Self {
        Parser {
            index,
            tokens,
            stack: Vec::new(), // with_capacity(tokens.len()),
            parsed: Vec::with_capacity(tokens.len()),
        }
    }

    fn peek_token(&self) -> Option<Token<'t>> {
        let mut index = self.index;

        loop {
            if index >= self.tokens.len() - 1 {
                return None;
            }

            let next_token = self.tokens[index];
            index += 1;

            if let Token::Space(_) | Token::Newline(_) = next_token {
                continue;
            } else {
                return Some(next_token);
            }
        }
    }

    fn next_token(&mut self) -> Option<Token<'t>> {
        loop {
            if self.index > self.tokens.len() - 1 {
                return None;
            }

            let next_token = self.tokens[self.index];
            self.index += 1;

            if let Token::Space(_) | Token::Newline(_) = next_token {
                continue;
            } else {
                return Some(next_token);
            }
        }
    }

    fn consume_token(&mut self) {
        let _ = self.next_token();
        // loop {
        //     let next_token = self.tokens[self.index];
        //     self.index += 1;

        //     if let Token::Space(_) | Token::Newline(_) = next_token {
        //         continue;
        //     } else {
        //         break;
        //     }
        // }
    }

    fn push(&mut self, token: Token<'t>) {
        self.stack.push(token);
    }

    fn pop(&mut self) -> Option<Token<'t>> {
        self.stack.pop()
    }

    fn last(&mut self) -> Option<&Token<'t>> {
        self.stack.last()
    }

    fn write(&mut self, token: Token<'t>) {
        self.parsed.push(token);
    }

    /// Peeks at the next token and passes it into `predicate`.
    ///
    /// When `predicate` returns `true`, this function returns `Ok` containing the peeked token. Otherwise, this function returns `Err`.
    fn expect(
        &mut self,
        predicate: impl FnOnce(&Token<'t>) -> bool,
    ) -> Result<Token<'t>, UnexpectedToken> {
        match self.peek_token() {
            Some(token) if predicate(&token) => Ok(token),
            Some(token) => {
                let start = token.loc();
                let length = token.as_str().len();

                Err(UnexpectedToken {
                    found: Some(token.as_str().to_owned()),
                    unexpected_span: LabeledSpan::new(None, start, length),
                })
            }
            None => {
                self.index -= 1;
                let last_token = self.peek_token();

                match last_token {
                    Some(token) => {
                        trace!("Erroring out, last_token: {:?}", token);
                        let start = token.loc();
                        let length = if token.spaced() {
                            token.as_str().len() + 1
                        } else {
                            token.as_str().len()
                        };

                        Err(UnexpectedToken {
                            found: None,
                            unexpected_span: LabeledSpan::new(None, start + length, 0),
                        })
                    }
                    None => Err(UnexpectedToken {
                        found: None,
                        unexpected_span: LabeledSpan::new(None, 0, 0),
                    }),
                }
            }
        }
    }

    /// Peeks at the next token and passes it into `predicate`.
    ///
    /// When `predicate` returns `true`, this function returns `Ok` containing the peeked token. Otherwise, this function returns a `miette::Err` decorated with `msg`.
    fn expect_with_msg(
        &mut self,
        predicate: impl FnOnce(&Token<'t>) -> bool,
        msg: impl FnOnce(&UnexpectedToken) -> String,
    ) -> miette::Result<Token<'t>> {
        let result = self.expect(predicate);

        match result {
            Ok(token) => Ok(token),
            Err(err) => {
                let msg = msg(&err);
                let span = err.unexpected_span.clone();
                let description = err.to_string();

                let labelled_span =
                    LabeledSpan::new(Some(msg.to_string()), span.offset(), span.len());

                Err(miette!(labels = vec![labelled_span], "{description}"))
            }
        }
    }

    pub fn parse<'p>(mut self) -> miette::Result<Vec<Token<'t>>> {
        trace!("Start parsing");

        let token = self.peek_token().wrap_err_with(|| "Expected token")?;

        match token {
            _ => {
                expression(&mut self)?;
            }
        };

        Ok(self.parsed)
    }
}

fn expression<'p, 't>(parser: &'p mut Parser<'t>) -> miette::Result<()> {
    // let mut last_token = Token::Unknown(Inner::new(0, ""));
    let mut last_rp = 0; // Right precedence of the operator just to the left of our current position
    loop {
        let token = parser.next_token();
        if let None = token {
            trace!("Found None");
            return Ok(());
        }
        let token = token.unwrap();
        trace!("Found {}", &token);

        // if token.is_operator() || token.is_initial() {
        match token {
            // Write idents/literals immediately
            Token::Identifier(_) | Token::NumericLiteral(_) => {
                parser.write(token);
            }

            // Handle end of expression
            Token::SemiColon(_) => {
                trace!("End of expression");
                while let Some(popped) = parser.pop() {
                    parser.write(popped);
                }
                parser.write(token);
                return Ok(());
            }

            // If the incoming token is an intial, push it to the stack
            Token::OpenBracket(_) => {
                let (_, rp) = token.precedence();

                parser.write(token); // Write immediately so that it starts the expression it bounds

                parser.push(token); // If incoming token is an initial, push to stack
                last_rp = rp;
            }

            // If the incoming token is a terminal, write it, and pop and write the stack until you see the corresponding initial token
            Token::CloseBracket(_) => {
                while let Some(popped) = parser.pop() {
                    match popped {
                        Token::OpenBracket(_) => {
                            break;
                        }
                        _ => {
                            parser.write(popped);
                        }
                    }
                }

                if let Some(token) = parser.last() {
                    let (_, rp) = token.precedence();
                    last_rp = rp;
                } else {
                    last_rp = 0;
                }

                parser.write(token);
            }

            _ => {
                let (lp, rp) = token.precedence();
                if lp < last_rp {
                    // If the incoming operator has lower lp than the last rp on the stack, pop and write until this is no longer the case
                    while let Some(popped) = parser.pop() {
                        let (_pop_lp, pop_rp) = popped.precedence();
                        parser.write(popped);
                        if lp < pop_rp {
                            break;
                        }
                    }
                }
                // If the incoming operator has higher lp than the last rp on top of the stack, push
                parser.push(token);
                last_rp = rp;
            }
        }

        trace!(
            "LRP: {}, Stack: {}, Parsed: {}",
            last_rp,
            parsed_to_str(&parser.stack),
            parsed_to_str(&parser.parsed)
        );
    }
}

// fn identifier_or_literal<'t>(parser: &mut Parser<'t>) -> miette::Result<Token<'t>> {
//     trace!("Parsing identifier or literal");

//     let token = parser.expect_with_msg(
//         |token| matches!(token, Token::Identifier(_) | Token::NumericLiteral(_)),
//         |found| format!("Expected identifier or literal, found {found}"),
//     )?;

//     trace!("Got {:?}", token);

//     parser.push(token);
//     Ok(token)
// }

// fn plus<'t>(parser: &mut Parser<'t>) -> miette::Result<()> {
//     trace!("Parsing plus");

//     let expression = expression(parser)?;

//     let next_token = parser.next_token();
//     while let Some(Token::Plus(inner)) = next_token {
//         plus(parser)?;
//         parser.push(Token::Plus(inner));
//     }

//     Ok(())
// }

// ------------- Older

// pub fn parse<'a>(parser: &mut Parser<'a>) -> miette::Result<Vec<Token<'a>>> {
//     loop {
//         let token = parser.peek_token();
//
//         if token.is_none() {
//             break;
//         }
//
//         let token = token.unwrap();
//
//         // Parse top level tokens
//         match token {
//             Token::Identifier(_) => {
//                 expression(parser, 0)?;
//             }
//             Token::FunctionDeclaration(_) => {
//                 function_declaration(parser)?;
//             }
//             _ => {
//                 bail!("Unexpected token {:?}", token);
//             }
//         }
//     }
//
//     Ok(parser.stack.clone())
// }
//
// fn function_declaration(parser: &mut Parser<'_>) -> miette::Result<()> {
//     trace!("Parsing function declaration");
//     parser.expect_with_msg(
//         |token| matches!(token, Token::FunctionDeclaration(_)),
//         |_| "Expected function declaration".into(),
//     )?;
//
//     let ident = parser.expect_with_msg(
//         |token| matches!(token, Token::Identifier(_)),
//         |err| {
//             let found = err.found.clone().unwrap_or("None".to_owned());
//             format!("Expected identifier, found {found}")
//         },
//     )?;
//
//     trace!("Found identifier {:?}", &ident);
//
//     let open_bracket = parser.expect_with_msg(
//         |token| matches!(token, Token::OpenBracket(_)),
//         |_| "Expected (".into(),
//     )?;
//     parser.stack.push(open_bracket);
//
//     // Parse parameters
//     expression(parser, 0)?;
//
//     let close_bracket = parser.expect_with_msg(
//         |token| matches!(token, Token::CloseBracket(_)),
//         |_| "Expected )".into(),
//     )?;
//     parser.stack.push(close_bracket);
//
//     // Parse return type
//     expression(parser, 0)?;
//
//     let curly_or_equals = parser.stack.last();
//
//     match curly_or_equals {
//         Some(Token::OpenCurly(_)) => {
//             parser.stack.push(*curly_or_equals.unwrap());
//
//             // TODO: Parse function body
//             expression(parser, 0)?;
//
//             let close_curly = parser.next_token();
//
//             match close_curly {
//                 Some(Token::CloseCurly(_)) => {
//                     parser.stack.push(close_curly.unwrap());
//                 }
//                 None => {
//                     bail!(
//                         "Expected end of function body after {:?}",
//                         parser.stack.last()
//                     );
//                 }
//                 _ => {
//                     bail!("Expected }} found {:?}", close_curly.unwrap());
//                 }
//             }
//         }
//         Some(Token::Equals(_)) => {
//             parser.stack.push(*curly_or_equals.unwrap());
//             expression(parser, 0)?;
//         }
//         None => {
//             let found = *curly_or_equals.unwrap();
//             bail!(
//                 "Expected function body or expression after {:?}, found {:?}",
//                 parser.stack.pop().unwrap(),
//                 found,
//             );
//         }
//         _ => {
//             let found = *curly_or_equals.unwrap();
//             bail!(
//                 "Expected {{ or = after {:?}, found {:?}",
//                 parser.stack.pop().unwrap(),
//                 found,
//             );
//         }
//     }
//
//     Ok(())
// }
//
// fn expression(parser: &mut Parser<'_>, precedence: u8) -> miette::Result<()> {
//     loop {
//         let token = parser.next_token();
//
//         match token {
//             // End of expression
//             Some(Token::SemiColon(_) | Token::OpenCurly(_) | Token::CloseBracket(_)) => {
//                 parser.stack.push(token.unwrap());
//                 break;
//             }
//             Some(token) => {
//                 token.parse(parser)?;
//             }
//             None => {
//                 bail!("Unexpected end of expression after {:?}", parser.last());
//             }
//         }
//     }
//
//     trace!("Parsed expression, stack: {:?}", &parser.stack);
//
//     // let terminator = parser.next_token().unwrap();
//
//     // parser.stack.push(terminator);
//
//     Ok(())
// }
