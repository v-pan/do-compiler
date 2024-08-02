// use crate::error::TokenizationError;
use std::{
    io::Seek,
    io::{BufReader, Read, SeekFrom},
};
use token_macro::TokenTypeDef;

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub loc: usize,
    pub ty: TokenType,
}

impl Token {
    pub fn new(loc: usize, ty: TokenType) -> Self {
        Token { loc, ty }
    }

    pub fn new_word(loc: usize, word: &str) -> Self {
        let token_type = TokenType::from(word);
        Token {
            loc,
            ty: token_type,
        }
    }

    pub fn new_char(loc: usize, word: char) -> Self {
        let token_type = TokenType::from(word);
        Token {
            loc,
            ty: token_type,
        }
    }

    pub fn get_string<T: std::io::Read + Seek>(
        &self,
        tokens: &[Token],
        reader: &mut BufReader<T>,
    ) -> String {
        let idx = tokens
            .binary_search_by(|other| other.loc.cmp(&self.loc))
            .expect("Did not find token");
        let pos = SeekFrom::Start(self.loc.try_into().unwrap());

        let next = tokens.get(idx + 1);
        reader.seek(pos).expect("Failed to seek to token start");

        if let Some(token) = next {
            let len = token
                .loc
                .checked_sub(self.loc)
                .expect("Overflow occurred while getting token length");
            let mut buf = vec![0_u8; len];

            reader.read_exact(&mut buf).unwrap();

            // println!("Byte len: {len}, vec len: {}, buf: {:?}", buf.len(), buf);

            String::from_utf8(buf).unwrap()
        } else {
            let mut buf = vec![];
            reader.read_to_end(&mut buf).unwrap();
            String::from_utf8(buf).unwrap()
        }
    }
}

#[derive(TokenTypeDef, Clone, Copy, Debug)]
pub enum TokenType {
    // Keywords
    #[word = "fun"]
    #[pair(SemiColon)]
    FunctionDecl,
    #[word = "if"]
    If,
    #[word = "val"]
    #[pair(SemiColon)]
    ValueDecl,
    #[word = "var"]
    #[pair(SemiColon)]
    VariableDecl,

    // Parentheses
    #[char = '(']
    #[pair(CloseParen)]
    OpenParen,
    #[char = ')']
    #[pair(OpenParen)]
    CloseParen,
    #[char = '{']
    #[pair(CloseCurly)]
    OpenCurly,
    #[char = '}']
    #[pair(OpenCurly)]
    CloseCurly,
    #[char = '<']
    #[pair(CloseAngle)]
    OpenAngle, // The angle brackets are also technically operators, context depending
    #[char = '>']
    #[pair(OpenAngle)]
    CloseAngle,

    // Quotes
    #[char = r#"""#]
    #[pair(DoubleQuote)]
    DoubleQuote,
    #[char = '\'']
    #[pair(SingleQuote)]
    SingleQuote,
    #[char = '`']
    #[pair(Backtick)]
    Backtick,

    // Seperators
    #[char = '=']
    Equals,
    #[char = ':']
    Colon,
    #[char = ',']
    Comma,
    #[char = ';']
    SemiColon,

    // Operators (excl. angle brackets, see above)
    #[char = '+']
    #[operator(precedence = 1)]
    Plus,
    #[char = '-']
    #[operator(precedence = 1)]
    Minus,
    #[char = '*']
    #[operator(precedence = 2)]
    Star,
    #[char = '/']
    #[operator(precedence = 2)]
    Slash,

    // Whitespace
    #[char = ' ']
    Space,
    #[char = '\n']
    Newline,

    // Comments - Currently think comments aren't being split on, but will be tokenized as slashes and stars
    // LineComment,
    // OpenMultilineComment,
    // CloseMultilineComment,

    // Unknown: Either an identifier or literal
    Unknown,

    Identifier,
    Literal,
    StringLiteral,
}

impl TokenType {
    pub fn is_introducer(&self) -> bool {
        match self {
            // Keywords
            TokenType::FunctionDecl => true,
            TokenType::If => true,
            TokenType::ValueDecl => true,
            TokenType::VariableDecl => true,
            // Parentheses
            TokenType::OpenParen => true,
            TokenType::OpenCurly => true,
            TokenType::OpenAngle => true,
            TokenType::CloseParen => false,
            TokenType::CloseCurly => false,
            TokenType::CloseAngle => false,
            // Quotes
            TokenType::DoubleQuote => true,
            TokenType::SingleQuote => true,
            TokenType::Backtick => true,
            // Seperators
            TokenType::Colon => false,
            TokenType::Comma => false,
            TokenType::SemiColon => false,
            TokenType::Equals => false,
            // Operators (excl. angle brackets, see above)
            TokenType::Plus => false,
            TokenType::Minus => false,
            TokenType::Star => false,
            TokenType::Slash => false,
            // Whitespace
            TokenType::Space => false,
            TokenType::Newline => false,
            // Unknown: Either an identifier or literal
            TokenType::Unknown => false,
            TokenType::Identifier => false,
            TokenType::Literal => false,
            TokenType::StringLiteral => false,
        }
    }

    pub fn is_whitespace(&self) -> bool {
        match self {
            // Keywords
            TokenType::FunctionDecl => false,
            TokenType::If => false,
            TokenType::ValueDecl => false,
            TokenType::VariableDecl => false,
            // Parentheses
            TokenType::OpenParen => false,
            TokenType::OpenCurly => false,
            TokenType::OpenAngle => false,
            TokenType::CloseParen => false,
            TokenType::CloseCurly => false,
            TokenType::CloseAngle => false,
            // Quotes
            TokenType::DoubleQuote => true,
            TokenType::SingleQuote => true,
            TokenType::Backtick => true,
            // Seperators
            TokenType::Equals => false,
            TokenType::Colon => false,
            TokenType::Comma => false,
            TokenType::SemiColon => false,
            // Operators (excl. angle brackets, see above)
            TokenType::Plus => false,
            TokenType::Minus => false,
            TokenType::Star => false,
            TokenType::Slash => false,
            // Whitespace
            TokenType::Space => true,
            TokenType::Newline => true,
            // Unknown: Either an identifier or literal
            TokenType::Unknown => false,
            TokenType::Identifier => false,
            TokenType::Literal => false,
            TokenType::StringLiteral => false,
        }
    }

    pub fn is_quote(&self) -> bool {
        matches!(&self, TokenType::DoubleQuote | TokenType::SingleQuote)
    }
}
