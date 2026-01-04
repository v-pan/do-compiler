use std::{
    fmt::{self, Formatter},
};

#[derive(Debug, Clone, Copy, token_macro::Token)]
pub enum Token<'a> {
    // --- Operators ---
    // Arithmetic Operators
    #[operator]
    #[word = "+"]
    Plus(Inner<'a>),

    #[operator]
    #[word = "-"]
    Minus(Inner<'a>),

    #[operator]
    #[word = "*"]
    Times(Inner<'a>),

    // Comparison Operators
    #[word = ">"]
    GreaterThan(Inner<'a>),
    #[word = "="]
    Equals(Inner<'a>),

    // Misc Operators
    #[word = ":"]
    Colon(Inner<'a>),
    #[word = ","]
    Comma(Inner<'a>),
    #[word = "->"]
    Arrow(Inner<'a>),

    // --- Initial / Terminal tokens ---
    #[initial]
    #[word = "fn"]
    FunctionDeclaration(Inner<'a>),
    #[terminal]
    #[word = ";"]
    SemiColon(Inner<'a>),

    #[initial]
    #[word = "("]
    OpenBracket(Inner<'a>),
    #[terminal]
    #[word = ")"]
    CloseBracket(Inner<'a>),
    #[initial]
    #[word = "{"]
    OpenCurly(Inner<'a>),
    #[terminal]
    #[word = "}"]
    CloseCurly(Inner<'a>),

    // --- Whitespace ---
    #[word = " "]
    Space(Inner<'a>),
    #[word = "\n"]
    Newline(Inner<'a>),

    // --- Literals / Identifiers ---
    NumericLiteral(Inner<'a>),
    Identifier(Inner<'a>),
    Unknown(Inner<'a>),
}

#[derive(Debug, Clone, Copy)]
pub struct Inner<'a> {
    pub loc: usize,
    pub slice: &'a str,
    pub spaced: bool,
}

impl<'a> Inner<'a> {
    pub fn new(loc: usize, slice: &'a str) -> Self {
        Self {
            loc,
            slice,
            spaced: false,
        }
    }
}

impl<'a> Token<'a> {
    pub fn precedence(&self) -> (u8, u8) {
        match self {
            // Operators
            Token::OpenBracket(_) => (1, 2),
            Token::Plus(_) | Token::Minus(_) => (3, 4),
            Token::Times(_) => (5, 6),

            _ => (0, 0),
        }
    }

}

impl<'t> fmt::Display for Token<'t> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let inner = self.inner();
        let spacing = if inner.spaced { " " } else { "" };
        let slice = inner.slice.to_string();
        write!(f, "{}{}", slice, spacing)
    }
}
