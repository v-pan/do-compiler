use log::{debug, trace};
use miette::{bail, diagnostic, miette, Context, LabeledSpan, MietteDiagnostic, SourceSpan};

use crate::token::Token;

use super::error::UnexpectedToken;

pub struct Parser<'a> {
    index: usize,
    tokens: &'a [Token<'a>],
    pub stack: Vec<Token<'a>>,
    parsed: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(index: usize, tokens: &'a [Token<'a>]) -> Self {
        Parser {
            index,
            tokens,
            stack: vec![],
            parsed: Vec::with_capacity(tokens.len()),
        }
    }

    pub fn peek_token(&self) -> Option<Token<'a>> {
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

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        loop {
            if self.index >= self.tokens.len() - 1 {
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

    pub fn consume_token(&mut self) {
        loop {
            let next_token = self.tokens[self.index];
            self.index += 1;

            if let Token::Space(_) | Token::Newline(_) = next_token {
                continue;
            } else {
                break;
            }
        }
    }

    pub fn push(&mut self, token: Token<'a>) {
        self.stack.push(token);
    }

    pub fn pop(&mut self) -> Option<Token<'a>> {
        self.stack.pop()
    }

    pub fn last(&mut self) -> Option<&Token<'a>> {
        self.stack.last()
    }

    pub fn expect(
        &mut self,
        condition: impl FnOnce(&Token<'a>) -> bool,
    ) -> Result<Token<'a>, UnexpectedToken> {
        match self.next_token() {
            Some(token) if condition(&token) => Ok(token),
            Some(token) => {
                let start = token.loc();
                let length = token.as_str().len();

                Err(UnexpectedToken {
                    found: token.as_str().to_owned(),
                    unexpected_span: LabeledSpan::new(None, start, length),
                })
            }
            None => Ok(Token::from(0, "test")),
        }
    }

    pub fn expect_with_msg(
        &mut self,
        condition: impl FnOnce(&Token<'a>) -> bool,
        msg: impl FnOnce(&UnexpectedToken) -> String,
    ) -> miette::Result<Token<'a>> {
        let result = self.expect(condition);

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
}

pub fn parse<'a>(parser: &mut Parser<'a>) -> miette::Result<Vec<Token<'a>>> {
    loop {
        let token = parser.peek_token();

        if token.is_none() {
            break;
        }

        let token = token.unwrap();

        match token {
            Token::Identifier(_) => {
                expression(parser, 0)?;
            }
            Token::FunctionDeclaration(_) => {
                function_declaration(parser)?;
            }
            _ => {
                bail!("Unexpected token {:?}", token);
            }
        }
    }

    Ok(parser.stack.clone())
}

fn function_declaration(parser: &mut Parser<'_>) -> miette::Result<()> {
    trace!("Parsing function declaration");
    parser.expect_with_msg(
        |token| matches!(token, Token::FunctionDeclaration(_)),
        |_| "Expected function declaration".into(),
    )?;

    let ident = parser.expect_with_msg(
        |token| matches!(token, Token::Identifier(_)),
        |err| {
            let found = err.found.clone();
            format!("Expected identifier, found {found}")
        },
    )?;

    trace!("Found identifier {:?}", &ident);

    let open_bracket = parser.expect_with_msg(
        |token| matches!(token, Token::OpenBracket(_)),
        |_| "Expected (".into(),
    )?;
    parser.stack.push(open_bracket);

    // Parse parameters
    expression(parser, 0)?;

    // Parse return type
    expression(parser, 0)?;

    let curly_or_equals = parser.stack.last();

    match curly_or_equals {
        Some(Token::OpenCurly(_)) => {
            parser.stack.push(*curly_or_equals.unwrap());

            // TODO: Parse function body
            expression(parser, 0)?;

            let close_curly = parser.next_token();

            match close_curly {
                Some(Token::CloseCurly(_)) => {
                    parser.stack.push(close_curly.unwrap());
                }
                None => {
                    bail!(
                        "Expected end of function body after {:?}",
                        parser.stack.last()
                    );
                }
                _ => {
                    bail!("Expected }} found {:?}", close_curly.unwrap());
                }
            }
        }
        Some(Token::Equals(_)) => {
            parser.stack.push(*curly_or_equals.unwrap());
            expression(parser, 0)?;
        }
        None => {
            let found = *curly_or_equals.unwrap();
            bail!(
                "Expected function body or expression after {:?}, found {:?}",
                parser.stack.pop().unwrap(),
                found,
            );
        }
        _ => {
            let found = *curly_or_equals.unwrap();
            bail!(
                "Expected {{ or = after {:?}, found {:?}",
                parser.stack.pop().unwrap(),
                found,
            );
        }
    }

    Ok(())
}

fn expression(parser: &mut Parser<'_>, precedence: u8) -> miette::Result<()> {
    loop {
        let token = parser.next_token();

        match token {
            // End of expression
            Some(Token::SemiColon(_) | Token::OpenCurly(_) | Token::CloseBracket(_)) => {
                break;
            }
            Some(token) => {
                token.parse(parser)?;
            }
            None => {
                bail!("Unexpected end of expression after {:?}", parser.last());
            }
        }
    }

    trace!("Parsed expression, stack: {:?}", &parser.stack);

    // let terminator = parser.next_token().unwrap();

    // parser.stack.push(terminator);

    Ok(())
}
