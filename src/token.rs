use packed_struct::prelude::*;


#[derive(PackedStruct)]
#[derive(Clone, Copy, Debug)]
#[packed_struct(bits=32, bit_numbering="msb0")]
pub struct Token {
    #[packed_field(bits="0..8")]
    line: u8,
    #[packed_field(bits="8..16")]
    loc: u8,
    #[packed_field(bits="16..32", endian="msb", ty="enum")]
    ty: TokenType
}

impl Token {
    pub fn new(line: u8, loc: u8, ty: TokenType) -> Self {
        Token { line, loc, ty }
    }

    pub fn try_keyword(line: u8, loc: u8, word: &str) -> Option<Token> {
        match word {
            "fun" => Some(Token { line, loc, ty: TokenType::Function }),
            "if" => Some(Token { line, loc, ty: TokenType::If }),
            _ => None
        }
    }

    pub fn try_paren(line: u8, loc: u8, word: &str) -> Option<Token> {
        match word {
            "(" => Some(Token { line, loc, ty: TokenType::OpenParen }),
            ")" => Some(Token { line, loc, ty: TokenType::CloseParen }),
            "<" => Some(Token { line, loc, ty: TokenType::OpenAngle }),
            ">" => Some(Token { line, loc, ty: TokenType::CloseAngle }),
            "{" => Some(Token { line, loc, ty: TokenType::OpenCurly }),
            "}" => Some(Token { line, loc, ty: TokenType::CloseCurly }),
            _ => None
        }
    }

    pub fn try_operator(line: u8, loc: u8, word: &str) -> Option<Token> {
        match word {
            "+" => Some(Token { line, loc, ty: TokenType::Plus }),
            "-" => Some(Token { line, loc, ty: TokenType::Minus }),
            "*" => Some(Token { line, loc, ty: TokenType::Star }),
            "/" => Some(Token { line, loc, ty: TokenType::Slash }),
            _ => None
        }
    }


    pub fn try_seperator(line: u8, loc: u8, word: &str) -> Option<Token> {
        match word {
            ":" => Some(Token { line, loc, ty: TokenType::TypeSeperator }),
            "," => Some(Token { line, loc, ty: TokenType::Comma }),
            _ => None
        }
    }

    pub fn try_quote(line: u8, loc: u8, word: &str) -> Option<Token> {
        match word {
            "\"" => Some(Token { line, loc, ty: TokenType::DoubleQuote }),
            "\'" => Some(Token { line, loc, ty: TokenType::SingleQuote }),
            "`" => Some(Token { line, loc, ty: TokenType::Backtick }),
            _ => None
        }
    }

    pub fn try_whitespace(line: u8, loc: u8, word: &str) -> Option<Token> {
        match word {
            " " => Some(Token { line, loc, ty: TokenType::Space }),
            "\n" => Some(Token { line, loc, ty: TokenType::Newline }),
            "\r\n" => Some(Token { line, loc, ty: TokenType::Newline }),
            _ => None
        }
    }
}

#[derive(PrimitiveEnum_u16)]
#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    // Keywords
    Function,
    If,

    // Parentheses
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    OpenAngle, // The angle brackets are also technically operators, context depending
    CloseAngle,

    // Quotes
    DoubleQuote,
    SingleQuote,
    Backtick,

    // Seperators
    TypeSeperator,
    Comma,

    // Operators (excl. angle brackets, see above)
    Plus,
    Minus,
    Star,
    Slash,

    // Whitespace
    Space,
    Newline,

    // Comments - Currently think comments aren't being split on, but will be tokenized as slashes and stars
    // LineComment,
    // OpenMultilineComment, 
    // CloseMultilineComment,

    // Unknown: Either an identifier or literal
    Unknown,
}
