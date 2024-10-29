use log::trace;
use miette::bail;

use crate::parse::parser::Parser;

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    // Operators
    Plus(Inner<'a>),
    Minus(Inner<'a>),

    GreaterThan(Inner<'a>),
    Equals(Inner<'a>),

    Colon(Inner<'a>),
    Comma(Inner<'a>),
    Arrow(Inner<'a>),

    // Brackets
    FunctionDeclaration(Inner<'a>),
    SemiColon(Inner<'a>),

    OpenBracket(Inner<'a>),
    CloseBracket(Inner<'a>),
    OpenCurly(Inner<'a>),
    CloseCurly(Inner<'a>),

    // Whitespace
    Space(Inner<'a>),
    Newline(Inner<'a>),

    Identifier(Inner<'a>),
    Unknown(Inner<'a>),
}

#[derive(Debug, Clone, Copy)]
pub struct Inner<'a> {
    pub loc: usize,
    pub slice: &'a str,
}

impl<'a> Token<'a> {
    pub fn from(loc: usize, slice: &'a str) -> Self {
        match slice {
            // Operators
            "+" => Token::Plus(Inner { loc, slice }),
            "-" => Token::Minus(Inner { loc, slice }),

            ">" => Token::GreaterThan(Inner { loc, slice }),
            "=" => Token::Equals(Inner { loc, slice }),

            ":" => Token::Colon(Inner { loc, slice }),
            "," => Token::Comma(Inner { loc, slice }),
            "->" => Token::Arrow(Inner { loc, slice }),

            // Brackets
            ";" => Token::SemiColon(Inner { loc, slice }),
            "func" => Token::FunctionDeclaration(Inner { loc, slice }),

            "(" => Token::OpenBracket(Inner { loc, slice }),
            ")" => Token::CloseBracket(Inner { loc, slice }),
            "{" => Token::OpenCurly(Inner { loc, slice }),
            "}" => Token::CloseCurly(Inner { loc, slice }),

            // Whitespace
            " " => Token::Space(Inner { loc, slice }),
            "\n" => Token::Newline(Inner { loc, slice }),

            _ => Token::Identifier(Inner { loc, slice }),
        }
    }

    pub fn loc(&self) -> usize {
        match self {
            Token::Plus(inner)
            | Token::Minus(inner)
            | Token::GreaterThan(inner)
            | Token::Equals(inner)
            | Token::Colon(inner)
            | Token::Comma(inner)
            | Token::Arrow(inner)
            | Token::FunctionDeclaration(inner)
            | Token::SemiColon(inner)
            | Token::OpenBracket(inner)
            | Token::CloseBracket(inner)
            | Token::OpenCurly(inner)
            | Token::CloseCurly(inner)
            | Token::Space(inner)
            | Token::Newline(inner)
            | Token::Identifier(inner)
            | Token::Unknown(inner) => inner.loc,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Token::Plus(inner)
            | Token::Minus(inner)
            | Token::GreaterThan(inner)
            | Token::Equals(inner)
            | Token::Colon(inner)
            | Token::Comma(inner)
            | Token::Arrow(inner)
            | Token::FunctionDeclaration(inner)
            | Token::SemiColon(inner)
            | Token::OpenBracket(inner)
            | Token::CloseBracket(inner)
            | Token::OpenCurly(inner)
            | Token::CloseCurly(inner)
            | Token::Space(inner)
            | Token::Newline(inner)
            | Token::Identifier(inner)
            | Token::Unknown(inner) => inner.slice,
        }
    }

    pub fn precedence(&self) -> (u8, u8) {
        match self {
            // Operators
            Token::Equals(_) => (2, 1),

            Token::Plus(_) => (3, 4),

            Token::Colon(_) => (3, 4),
            Token::Comma(_) => (2, 1),

            _ => (0, 0),
        }
    }

    pub fn parse(&self, parser: &mut Parser<'a>) -> miette::Result<()> {
        match &self {
            Token::Plus(_)
            | Token::Minus(_)
            | Token::Equals(_)
            | Token::GreaterThan(_)
            | Token::Colon(_)
            | Token::Comma(_)
            | Token::Arrow(_) => {
                binary_operator(*self, parser)?;
            }
            Token::FunctionDeclaration(_)
            | Token::SemiColon(_)
            | Token::OpenBracket(_)
            | Token::CloseBracket(_)
            | Token::OpenCurly(_)
            | Token::CloseCurly(_) => {}
            Token::Identifier(_) => {
                trace!("Parsing identifier {:?}", self);
                parser.push(*self);
            }
            Token::Newline(_) | Token::Space(_) => {}
            Token::Unknown(_) => {}
        }

        Ok(())
    }
}

fn binary_operator<'a>(operator: Token<'a>, parser: &mut Parser<'a>) -> miette::Result<()> {
    trace!("Parsing operator {:?}", operator);
    let token = parser.next_token();

    match token {
        Some(rhs) => {
            // Check if this is just an argument
            if let Token::Identifier(_) = rhs {
            } else {
                bail!("Expected identifier after {:?}", operator);
            }
            trace!("Found identifier {:?}", rhs);

            let lhs = parser.pop();
            match lhs {
                Some(Token::Identifier(_)) => {
                    parser.push(lhs.unwrap());
                    parser.push(rhs);
                    parser.push(operator);
                }
                Some(other_operator) => {
                    trace!(
                        "LHS operator: {:?}, RHS argument: {:?}",
                        other_operator,
                        rhs
                    );
                    let (_, other_rhs_precedence) = other_operator.precedence();

                    if other_rhs_precedence == 0 {
                        bail!(
                            "Expected operator as LHS argument to {:?}, found {:?}",
                            operator,
                            other_operator
                        );
                    }

                    if other_rhs_precedence < operator.precedence().0 {
                        // self should go beneath operator in the parse tree
                        parser.push(rhs);
                        parser.push(operator);
                        parser.push(other_operator);
                    } else {
                        // leave everything as is
                        parser.push(other_operator);
                        parser.push(rhs);
                        parser.push(operator);
                    }
                }
                None => {
                    bail!("Expected argument before binary operator {:?}", operator);
                }
            }
        }
        None => {
            bail!("Expected token after operator {:?}", operator);
        }
    }

    Ok(())
}
