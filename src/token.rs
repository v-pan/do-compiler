use log::{error, trace, warn};
// use crate::error::TokenizationError;
use miette::bail;
use std::{
    fmt::Debug,
    io::Seek,
    io::{BufReader, Read, SeekFrom},
};
use string_interner::backend::Backend;
use token_macro::TokenTypeDef;

use crate::parse::parser::Parser;

// #[derive(Clone, Copy)]
// pub struct Token<'a> {
//     pub loc: usize,
//     pub ty: TokenType,
//     pub slice: &'a str,
// }

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
    Space,
    Newline,

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
            " " => Token::Space,
            "\n" => Token::Newline,

            _ => Token::Identifier(Inner { loc, slice }),
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
                trace!("Parsing operator {:?}", self);
                let token = parser.next_token();

                match token {
                    Some(rhs) => {
                        // Check if this is just an argument
                        if let Token::Identifier(_) = rhs {
                        } else {
                            bail!("Expected identifier after {:?}", self);
                        }
                        trace!("Found identifier {:?}", rhs);

                        let lhs = parser.pop();
                        match lhs {
                            Some(Token::Identifier(_)) => {
                                parser.push(lhs.unwrap());
                                parser.push(rhs);
                                parser.push(*self);
                            }
                            Some(operator) => {
                                trace!("LHS operator: {:?}, RHS argument: {:?}", operator, rhs);
                                let (_, op_rhs_prec) = operator.precedence();

                                if op_rhs_prec == 0 {
                                    bail!(
                                        "Expected operator as LHS argument to {:?}, found {:?}",
                                        self,
                                        operator
                                    );
                                }

                                if op_rhs_prec < self.precedence().0 {
                                    // self should go beneath operator in the parse tree
                                    parser.push(rhs);
                                    parser.push(*self);
                                    parser.push(operator);
                                } else {
                                    // leave everything as is
                                    parser.push(operator);
                                    parser.push(rhs);
                                    parser.push(*self);
                                }
                            }
                            None => {
                                bail!("Expected argument before binary operator {:?}", self);
                            }
                        }
                    }
                    None => {
                        bail!("Expected token after operator {:?}", self);
                    }
                }
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
            Token::Newline | Token::Space => {}
            Token::Unknown(_) => {}
        }

        Ok(())
    }
}

// impl<'a> Clone for Token<'a> {
//     fn clone(&self) -> Self {
//         Token {
//             loc: self.loc.clone(),
//             ty: self.ty.clone(),
//             slice: self.slice.clone(),
//         }
//     }
// }

// impl<'a> Debug for Token<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Token")
//             .field("loc", &self.loc)
//             .field("ty", &self.ty)
//             .field("slice", &self.slice)
//             .finish()
//     }
// }
//
// impl<'a> Token<'a> {
//     pub fn new(loc: usize, ty: TokenType, slice: &'a str) -> Self {
//         Token { loc, ty, slice }
//     }
//
//     pub fn new_word(loc: usize, word: &'a str) -> Self {
//         let token_type = TokenType::from(word);
//
//         Token {
//             loc,
//             ty: token_type,
//             slice: word,
//         }
//     }
//
//     // pub fn get_string<T: std::io::Read + Seek>(
//     //     &self,
//     //     tokens: &[Token<B>],
//     //     reader: &mut BufReader<T>,
//     // ) -> String {
//     //     let idx = tokens
//     //         .binary_search_by(|other| other.loc.cmp(&self.loc))
//     //         .expect("Did not find token");
//     //     let pos = SeekFrom::Start(self.loc.try_into().unwrap());
//
//     //     let next = tokens.get(idx + 1);
//     //     reader.seek(pos).expect("Failed to seek to token start");
//
//     //     if let Some(token) = next {
//     //         let len = token
//     //             .loc
//     //             .checked_sub(self.loc)
//     //             .expect("Overflow occurred while getting token length");
//     //         let mut buf = vec![0_u8; len];
//
//     //         reader.read_exact(&mut buf).unwrap();
//
//     //         // println!("Byte len: {len}, vec len: {}, buf: {:?}", buf.len(), buf);
//
//     //         String::from_utf8(buf).unwrap()
//     //     } else {
//     //         let mut buf = vec![];
//     //         reader.read_to_end(&mut buf).unwrap();
//     //         String::from_utf8(buf).unwrap()
//     //     }
//     // }
// }
//
// #[derive(TokenTypeDef, Clone, Copy, Debug)]
// pub enum TokenType {
//     // Keywords
//     #[word = "fun"]
//     #[pair(SemiColon)]
//     FunctionDecl,
//     #[word = "if"]
//     If,
//     #[word = "val"]
//     #[pair(SemiColon)]
//     ValueDecl,
//     #[word = "var"]
//     #[pair(SemiColon)]
//     VariableDecl,
//
//     // Parentheses
//     #[word = "("]
//     #[pair(CloseParen)]
//     OpenParen,
//     #[word = ")"]
//     #[pair(OpenParen)]
//     CloseParen,
//     #[word = "{"]
//     #[pair(CloseCurly)]
//     OpenCurly,
//     #[word = "}"]
//     #[pair(OpenCurly)]
//     CloseCurly,
//     #[word = "<"]
//     #[pair(CloseAngle)]
//     OpenAngle, // The angle brackets are also technically operators, context depending
//     #[word = ">"]
//     #[pair(OpenAngle)]
//     CloseAngle,
//
//     // Quotes
//     #[char = r#"""#]
//     #[pair(DoubleQuote)]
//     DoubleQuote,
//     #[word = r#"'"#]
//     #[pair(SingleQuote)]
//     SingleQuote,
//     #[word = "`"]
//     #[pair(Backtick)]
//     Backtick,
//
//     // Seperators
//     #[word = "="]
//     Equals,
//     #[word = ":"]
//     Colon,
//     #[word = ","]
//     Comma,
//     #[word = ";"]
//     SemiColon,
//
//     // Operators (excl. angle brackets, see above)
//     #[word = "+"]
//     #[operator(precedence = 1)]
//     Plus,
//     #[word = "-"]
//     #[operator(precedence = 1)]
//     Minus,
//     #[word = "*"]
//     #[operator(precedence = 2)]
//     Star,
//     #[word = "/"]
//     #[operator(precedence = 2)]
//     Slash,
//
//     // Whitespace
//     #[word = " "]
//     Space,
//     #[word = "\n"]
//     Newline,
//
//     // Comments - Currently think comments aren't being split on, but will be tokenized as slashes and stars
//     // LineComment,
//     // OpenMultilineComment,
//     // CloseMultilineComment,
//
//     // Unknown: Either an identifier or literal
//     Unknown,
//
//     Identifier,
//     Literal,
//     StringLiteral,
// }
//
// impl TokenType {
//     pub fn is_introducer(&self) -> bool {
//         match self {
//             // Keywords
//             TokenType::FunctionDecl => true,
//             TokenType::If => true,
//             TokenType::ValueDecl => true,
//             TokenType::VariableDecl => true,
//             // Parentheses
//             TokenType::OpenParen => true,
//             TokenType::OpenCurly => true,
//             TokenType::OpenAngle => true,
//             TokenType::CloseParen => false,
//             TokenType::CloseCurly => false,
//             TokenType::CloseAngle => false,
//             // Quotes
//             TokenType::DoubleQuote => true,
//             TokenType::SingleQuote => true,
//             TokenType::Backtick => true,
//             // Seperators
//             TokenType::Colon => false,
//             TokenType::Comma => false,
//             TokenType::SemiColon => false,
//             TokenType::Equals => false,
//             // Operators (excl. angle brackets, see above)
//             TokenType::Plus => false,
//             TokenType::Minus => false,
//             TokenType::Star => false,
//             TokenType::Slash => false,
//             // Whitespace
//             TokenType::Space => false,
//             TokenType::Newline => false,
//             // Unknown: Either an identifier or literal
//             TokenType::Unknown => false,
//             TokenType::Identifier => false,
//             TokenType::Literal => false,
//             TokenType::StringLiteral => false,
//         }
//     }
//
//     pub fn is_whitespace(&self) -> bool {
//         match self {
//             // Keywords
//             TokenType::FunctionDecl => false,
//             TokenType::If => false,
//             TokenType::ValueDecl => false,
//             TokenType::VariableDecl => false,
//             // Parentheses
//             TokenType::OpenParen => false,
//             TokenType::OpenCurly => false,
//             TokenType::OpenAngle => false,
//             TokenType::CloseParen => false,
//             TokenType::CloseCurly => false,
//             TokenType::CloseAngle => false,
//             // Quotes
//             TokenType::DoubleQuote => true,
//             TokenType::SingleQuote => true,
//             TokenType::Backtick => true,
//             // Seperators
//             TokenType::Equals => false,
//             TokenType::Colon => false,
//             TokenType::Comma => false,
//             TokenType::SemiColon => false,
//             // Operators (excl. angle brackets, see above)
//             TokenType::Plus => false,
//             TokenType::Minus => false,
//             TokenType::Star => false,
//             TokenType::Slash => false,
//             // Whitespace
//             TokenType::Space => true,
//             TokenType::Newline => true,
//             // Unknown: Either an identifier or literal
//             TokenType::Unknown => false,
//             TokenType::Identifier => false,
//             TokenType::Literal => false,
//             TokenType::StringLiteral => false,
//         }
//     }
//
//     pub fn is_quote(&self) -> bool {
//         matches!(&self, TokenType::DoubleQuote | TokenType::SingleQuote)
//     }
// }
