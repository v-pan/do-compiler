// use crate::error::TokenizationError;
use std::{
    fmt::Debug,
    io::Seek,
    io::{BufReader, Read, SeekFrom},
};
use string_interner::backend::Backend;
use token_macro::TokenTypeDef;

// #[derive(Clone, Copy)]
// pub struct Token<'a> {
//     pub loc: usize,
//     pub ty: TokenType,
//     pub slice: &'a str,
// }

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Identifier(Inner<'a>),
    Plus(Inner<'a>),
    SemiColon(Inner<'a>),
    Space,
    Newline,

    Unknown(Inner<'a>),
}

#[derive(Debug, Clone, Copy)]
pub struct Inner<'a> {
    pub loc: usize,
    pub slice: &'a str,
}

impl<'a> Token<'a> {
    pub fn from(loc: usize, value: &'a str) -> Self {
        match value {
            "+" => Token::Plus(Inner { loc, slice: value }),
            ";" => Token::SemiColon(Inner { loc, slice: value }),
            " " => Token::Space,
            "\n" => Token::Newline,

            _ => Token::Identifier(Inner { loc, slice: value }),
        }
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
