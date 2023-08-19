use crate::parse::{Function, Parse, Node};
use crate::span::Span;

pub struct TokenLexer {
    contents: String,
    span: Span
}

impl TokenLexer {
    pub fn new(contents: String, span: Span) -> Self {
        TokenLexer { contents, span }
    }
}

impl Iterator for TokenLexer {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((remaining, _comment)) = self.contents.split_once("//") {
            self.contents = remaining.to_string();
        }
        if let Some((keyword, remaining)) = self.contents.split_once(" ") {
            match keyword {
                "fun" => {
                    let function = Function::parse(&mut self.contents, self.span).unwrap();
                    println!("Done parsing!");
                    println!("{}", &function.identifier.ident);
                    Some(Node::Fun(function))
                },
                _ => { Some(Node::Unknown) },
            }
        } else {
            None
        }
    }
}

