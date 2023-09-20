// use crate::error::TokenizationError;
use std::{io::{BufReader, SeekFrom, Read}, io::Seek, fs::File};
use token_macro::TokenTypeDef;

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub loc: u32,
    pub ty: TokenType
}

impl Token {
    pub fn new(loc: u32, word: &str) -> Self {
        Token::try_keyword(loc, word)
        .or(
            Token::try_paren(loc, word)
        ).or(
            Token::try_operator(loc, word)
        ).or(
            Token::try_seperator(loc, word)
        ).or(
            Token::try_whitespace(loc, word)
        ).or(
            Token::try_quote(loc, word)
        ).unwrap_or(
            Token { loc, ty: TokenType::Unknown }
        )
    }

    pub fn get_string(&self, tokens: &Vec<Token>, reader: &mut BufReader<&File>) -> String {
        let idx = tokens.binary_search_by(|other| { other.loc.cmp(&self.loc) }).expect("Did not find token");
        let pos = SeekFrom::Start(self.loc.try_into().unwrap());

        let next = tokens.get(idx+1);
        reader.seek(pos).expect("Failed to seek to token start");

        if let Some(token) = next {
            let len = token.loc.checked_sub(self.loc).expect("Overflow occurred while getting token length");
            let mut buf = vec![0_u8; len.try_into().unwrap()];

            reader.read_exact(&mut buf).unwrap();

            // println!("Byte len: {len}, vec len: {}, buf: {:?}", buf.len(), buf);

            String::from_utf8(buf).unwrap()
        } else {
            let mut buf = vec![];
            reader.read_to_end(&mut buf).unwrap();
            String::from_utf8(buf).unwrap()
        }
    }

    // pub fn get_string_packed(&self, tokens: &Vec<[u8;5]>, reader: &mut BufReader<&File>) -> String {
    //     let idx = tokens.binary_search(&self.pack().expect("Could not pack self")).expect("Did not find token");
    //     let pos = SeekFrom::Start(self.loc.try_into().unwrap());

    //     let next = tokens.get(idx+1);
    //     reader.seek(pos).expect("Failed to seek to token start");

    //     if let Some(token) = next {
    //         let len = Token::unpack_from_slice(token).unwrap().loc.checked_sub(self.loc).expect("Overflow occurred while getting token length");
    //         let mut buf = vec![0_u8; len.try_into().unwrap()];

    //         reader.read_exact(&mut buf).unwrap();

    //         // println!("Byte len: {len}, vec len: {}, buf: {:?}", buf.len(), buf);

    //         String::from_utf8(buf).unwrap()
    //     } else {
    //         let mut buf = vec![];
    //         reader.read_to_end(&mut buf).unwrap();
    //         String::from_utf8(buf).unwrap()
    //     }
    // }

    pub fn try_keyword(loc: u32, word: &str) -> Option<Token> {
        match word {
            "fun" => Some(Token { loc: loc.into(), ty: TokenType::FunctionDecl }),
            "if" => Some(Token { loc: loc.into(), ty: TokenType::If }),
            _ => None
        }
    }
    // pub fn try_keyword_packed(loc: u32, word: &str) -> Result<[u8; 5], TokenizationError> {
    //     Ok(Token::try_keyword(loc, word).ok_or(TokenizationError::NoMatch)?.pack()?)
    // }

    pub fn try_paren(loc: u32, word: &str) -> Option<Token> {
        match word {
            "(" => Some(Token { loc: loc.into(), ty: TokenType::OpenParen }),
            ")" => Some(Token { loc: loc.into(), ty: TokenType::CloseParen }),
            "<" => Some(Token { loc: loc.into(), ty: TokenType::OpenAngle }),
            ">" => Some(Token { loc: loc.into(), ty: TokenType::CloseAngle }),
            "{" => Some(Token { loc: loc.into(), ty: TokenType::OpenCurly }),
            "}" => Some(Token { loc: loc.into(), ty: TokenType::CloseCurly }),
            _ => None
        }
    }
    // pub fn try_paren_packed(loc: u32, word: &str) -> Result<[u8; 5], TokenizationError> {
    //     Ok(Token::try_paren(loc, word).ok_or(TokenizationError::NoMatch)?.pack()?)
    // }

    pub fn try_operator(loc: u32, word: &str) -> Option<Token> {
        match word {
            "+" => Some(Token { loc: loc.into(), ty: TokenType::Plus }),
            "-" => Some(Token { loc: loc.into(), ty: TokenType::Minus }),
            "*" => Some(Token { loc: loc.into(), ty: TokenType::Star }),
            "/" => Some(Token { loc: loc.into(), ty: TokenType::Slash }),
            _ => None
        }
    }
    // pub fn try_operator_packed(loc: u32, word: &str) -> Result<[u8; 5], TokenizationError> {
    //     Ok(Token::try_operator(loc, word).ok_or(TokenizationError::NoMatch)?.pack()?)
    // }

    pub fn try_seperator(loc: u32, word: &str) -> Option<Token> {
        match word {
            ":" => Some(Token { loc: loc.into(), ty: TokenType::Colon }),
            "," => Some(Token { loc: loc.into(), ty: TokenType::Comma }),
            ";" => Some(Token { loc: loc.into(), ty: TokenType::SemiColon }),
            _ => None
        }
    }
    // pub fn try_seperator_packed(loc: u32, word: &str) -> Result<[u8; 5], TokenizationError> {
    //     Ok(Token::try_seperator(loc, word).ok_or(TokenizationError::NoMatch)?.pack()?)
    // }

    pub fn try_quote(loc: u32, word: &str) -> Option<Token> {
        match word {
            "\"" => Some(Token { loc: loc.into(), ty: TokenType::DoubleQuote }),
            "\'" => Some(Token { loc: loc.into(), ty: TokenType::SingleQuote }),
            "`" => Some(Token { loc: loc.into(), ty: TokenType::Backtick }),
            _ => None
        }
    }
    // pub fn try_quote_packed(loc: u32, word: &str) -> Result<[u8; 5], TokenizationError> {
    //     Ok(Token::try_quote(loc, word).ok_or(TokenizationError::NoMatch)?.pack()?)
    // }

    pub fn try_whitespace(loc: u32, word: &str) -> Option<Token> {
        match word {
            " " => Some(Token { loc: loc.into(), ty: TokenType::Space }),
            "\n" => Some(Token { loc: loc.into(), ty: TokenType::Newline }),
            "\r\n" => Some(Token { loc: loc.into(), ty: TokenType::Newline }),
            _ => None
        }
    }
    // pub fn try_whitespace_packed(loc: u32, word: &str) -> Result<[u8; 5], TokenizationError> {
    //     Ok(Token::try_whitespace(loc, word).ok_or(TokenizationError::NoMatch)?.pack()?)
    // }
}

#[derive(TokenTypeDef, Clone, Copy, Debug)]
pub enum TokenType {
    // Keywords
    #[info(word="fun")]
    FunctionDecl,
    #[info(word="if")]
    If,

    // Parentheses
    #[info(word="(", group=CloseParen)]
    OpenParen,
    #[info(word=")", group=OpenParen)]
    CloseParen,
    #[info(word="{", group=CloseCurly)]
    OpenCurly,
    #[info(word="}", group=OpenCurly)]
    CloseCurly,
    #[info(word="<", group=CloseAngle)]
    OpenAngle, // The angle brackets are also technically operators, context depending
    #[info(word=">", group=OpenAngle)]
    CloseAngle,

    // Quotes
    #[info(word=r#"""#, group=DoubleQuote)]
    DoubleQuote,
    #[info(word="'", group=SingleQuote)]
    SingleQuote,
    #[info(word="`", group=Backtick)]
    Backtick,

    // Seperators
    #[info(word=":")]
    Colon,
    #[info(word=",")]
    Comma,
    #[info(word=";")]
    SemiColon,

    // Operators (excl. angle brackets, see above)
    #[info(word="+")]
    Plus,
    #[info(word="-")]
    Minus,
    #[info(word="*")]
    Star,
    #[info(word="/")]
    Slash,

    // Whitespace
    #[info(word=" ")]
    Space,
    #[info(word="\n")]
    Newline,

    // Comments - Currently think comments aren't being split on, but will be tokenized as slashes and stars
    // LineComment,
    // OpenMultilineComment, 
    // CloseMultilineComment,

    // Unknown: Either an identifier or literal
    Unknown,

    Identifier,
    Literal,
}

impl TokenType {
    pub fn is_introducer(&self) -> bool {
        match self {
            // Keywords
            TokenType::FunctionDecl => true,
            TokenType::If => true,
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
        }
    }

    pub fn is_whitespace(&self) -> bool {
        match self {
            // Keywords
            TokenType::FunctionDecl => false,
            TokenType::If => false,
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
        }
    }

    // If the given `TokenType` is an operator, returns its precedence and associativity.
    pub fn try_operator(&self) -> Option<(u8, Associativity)> {
        match self {
            // Keywords
            TokenType::FunctionDecl => None,
            TokenType::If => None,
            // Parentheses
            TokenType::OpenParen => None,
            TokenType::OpenCurly => None,
            TokenType::OpenAngle => None,
            TokenType::CloseParen => None,
            TokenType::CloseCurly => None,
            TokenType::CloseAngle => None,
            // Quotes
            TokenType::DoubleQuote => None,
            TokenType::SingleQuote => None,
            TokenType::Backtick => None,
            // Seperators
            TokenType::Colon => None,
            TokenType::Comma => None,
            TokenType::SemiColon => None,
            // Operators (excl. angle brackets, see above)
            TokenType::Plus => Some((1, Associativity::Left)),
            TokenType::Minus => Some((1, Associativity::Left)),
            TokenType::Star => Some((2, Associativity::Left)),
            TokenType::Slash => Some((2, Associativity::Left)),
            // Whitespace
            TokenType::Space => None,
            TokenType::Newline => None,
            // Unknown: Either an identifier or literal
            TokenType::Unknown => Some((4, Associativity::Right)), // Assumed to be a function call, treated as a unary operator
            TokenType::Identifier => None,
            TokenType::Literal => None,
        }
    }
}

pub enum Associativity {
    Left,
    Right
}