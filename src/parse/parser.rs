use log::{debug, trace};
use miette::bail;

use crate::token::Token;

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

            if let Token::Space | Token::Newline = next_token {
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

            if let Token::Space | Token::Newline = next_token {
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

            if let Token::Space | Token::Newline = next_token {
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
}

macro_rules! expect {
    ($token:expr, $type:path) => {
        match $token {
            Some($type(inner)) => $type(inner),
            None => {
                bail!(
                    "Expected token {:?}",
                    $type(crate::token::Inner { loc: 0, slice: "" })
                );
            }
            _ => {
                bail!(
                    "Expected token {:?}, found {:?}",
                    $type(crate::token::Inner { loc: 0, slice: "" }),
                    $token
                );
            }
        }
    };
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
    expect!(parser.next_token(), Token::FunctionDeclaration);

    let ident = expect!(parser.next_token(), Token::Identifier);
    trace!("Found identifier {:?}", &ident);

    let open_bracket = expect!(parser.next_token(), Token::OpenBracket);
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

fn identifier<'a>(parser: &mut Parser<'a>, ident: Token<'a>) {
    parser.stack.push(ident);
}

fn binary_operator<'a>(parser: &mut Parser<'a>, operator: Token<'a>) {
    parser.stack.push(operator);
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
