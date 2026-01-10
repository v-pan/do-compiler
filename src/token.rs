use std::fmt::{self, Formatter};

#[derive(Debug, Clone, Copy, token_macro::Token)]
pub enum Token<'buffer> {
    // --- Operators ---
    // Arithmetic Operators
    #[operator]
    #[word = "+"]
    Plus(Inner<'buffer>),

    #[operator]
    #[word = "-"]
    Minus(Inner<'buffer>),

    #[operator]
    #[word = "*"]
    Times(Inner<'buffer>),

    // Comparison Operators
    #[operator]
    #[word = ">"]
    GreaterThan(Inner<'buffer>),
    #[operator]
    #[word = "="]
    Equals(Inner<'buffer>),

    // Misc Operators
    #[operator]
    #[word = ":"]
    Colon(Inner<'buffer>),
    #[operator]
    #[word = ","]
    Comma(Inner<'buffer>),
    #[operator]
    #[word = "->"]
    Arrow(Inner<'buffer>),

    // --- Initial / Terminal tokens ---
    #[initial]
    #[word = "fn"]
    FunctionDeclaration(Inner<'buffer>),
    #[terminal]
    #[word = ";"]
    SemiColon(Inner<'buffer>),

    #[initial]
    #[word = "("]
    OpenBracket(Inner<'buffer>),
    #[terminal]
    #[word = ")"]
    CloseBracket(Inner<'buffer>),
    #[initial]
    #[word = "{"]
    OpenCurly(Inner<'buffer>),
    #[terminal]
    #[word = "}"]
    CloseCurly(Inner<'buffer>),

    // --- Whitespace ---
    #[word = " "]
    Space(Inner<'buffer>),
    #[word = "\n"]
    Newline(Inner<'buffer>),

    // --- Literals / Identifiers ---
    NumericLiteral(Inner<'buffer>),
    Identifier(Inner<'buffer>),
    Unknown(Inner<'buffer>),
}

#[derive(Debug, Clone, Copy)]
pub struct Inner<'buffer> {
    pub loc: usize,
    pub slice: &'buffer str,
    // pub spaced: bool,
}

impl<'buffer> Inner<'buffer> {
    pub fn new(loc: usize, slice: &'buffer str) -> Self {
        Self {
            loc,
            slice,
            // spaced: false,
        }
    }
}

impl<'a> Token<'a> {
    // TODO: Build precedence by transitively closing a graph of order relations
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

impl<'buffer> Default for Token<'buffer> {
    fn default() -> Self {
        Self::Unknown(Inner {
            loc: 0,
            slice: "default",
        })
    }
}

impl<'t> fmt::Display for Token<'t> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let inner = self.inner();
        // let spacing = if inner.spaced { " " } else { "" };
        let slice = inner.slice.to_string();
        write!(f, "{}", slice)
    }
}
