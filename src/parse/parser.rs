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

    fn peek_token(&self) -> Option<Token<'a>> {
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

    fn next_token(&mut self) -> Option<Token<'a>> {
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
        let token = parser.peek_token();

        if token.is_some() {
            trace!("Found token {:?}", &token.unwrap());
        }

        match token {
            // Arguments
            Some(Token::Identifier(_)) => {
                trace!("Token is identifier");

                identifier(parser, token.unwrap());
                parser.consume_token();

                trace!("Stack: {:?}", &parser.stack);
            }

            // Operators
            Some(Token::Plus(_))
            | Some(Token::Comma(_))
            | Some(Token::Colon(_))
            | Some(Token::Arrow(_)) => {
                trace!("Token is operator");

                let operator = token.unwrap();
                let (lhs_precedence, rhs_precedence) = operator.precedence();

                trace!(
                    "lhs_precedence: {}, rhs_precedence: {}, precedence: {}",
                    lhs_precedence,
                    rhs_precedence,
                    precedence
                );

                if lhs_precedence < precedence {
                    break;
                }

                parser.consume_token();

                // Want to find the next operator with higher precedence
                expression(parser, rhs_precedence)?;
                binary_operator(parser, operator);
            }

            // End of expression
            Some(Token::SemiColon(_))
            | Some(Token::CloseBracket(_))
            | Some(Token::OpenCurly(_)) => {
                break;
            }

            None => {
                bail!("Unexpected end of tokens. Did you forget to close the expression");
            }
            _ => {
                let token = token.unwrap();
                bail!(
                    "Expected argument, operator, or end of expression. Found {:?}",
                    &token
                );
            }
        }
    }

    trace!("Parsed expression, stack: {:?}", &parser.stack);

    let terminator = parser.next_token().unwrap();

    parser.stack.push(terminator);

    Ok(())
}
