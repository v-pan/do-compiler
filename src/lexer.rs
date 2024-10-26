use std::io::{BufReader, Read};
use string_interner::{
    backend::{Backend, StringBackend},
    StringInterner,
};

use crate::token::{Inner, Token};

pub struct AsciiLexer;

// pub struct AsciiLexer<B: string_interner::backend::Backend> {
//     pub idents: StringInterner<B>,
// }
//
// impl<B: string_interner::backend::Backend> AsciiLexer<B> {
//     pub fn new(idents: StringInterner<B>) -> AsciiLexer<B> {
//         AsciiLexer { idents }
//     }
// }
//
// impl Default for AsciiLexer<StringBackend> {
//     fn default() -> Self {
//         AsciiLexer {
//             idents: StringInterner::<StringBackend>::new(),
//         }
//     }
// }

impl<'a> AsciiLexer {
    pub fn new() -> Self {
        AsciiLexer
    }

    pub fn tokenize(&mut self, buf: &'a mut String) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        let mut last_token = Token::Unknown(Inner { loc: 0, slice: "" });
        let mut last_idx = 0;

        for (idx, byte) in buf.as_bytes().iter().enumerate() {
            // Convert to char unsafely to avoid slow validity checks
            let c: char = unsafe { char::from_u32_unchecked((*byte).into()) };

            if is_word_boundary(c) {
                // There might be tokens between us and the previous boundary
                if last_idx != idx {
                    // Get the str between both indices
                    let word: &str = unsafe { buf.get_unchecked(last_idx..idx) };

                    let word_token = Token::from(last_idx, word);

                    tokens.push(word_token);
                }

                let word: &str = unsafe { buf.get_unchecked(idx..idx + 1) };

                // Store the boundary token
                let token = Token::from(idx, word);

                // Look behind to see if this is a two character boundary token
                match token {
                    Token::GreaterThan(_) => {
                        if let Token::Minus(_) = last_token {
                            tokens.pop();
                            let token = Token::from(idx - 1, "->");

                            last_token = token;
                            tokens.push(token);
                        }
                    }
                    Token::Equals(_) => {
                        if let Token::Equals(_) = last_token {
                            // TODO: Comparison operators
                        }
                        last_token = token;
                        tokens.push(token);
                    }
                    _ => {
                        last_token = token;
                        tokens.push(token);
                    }
                }

                last_idx = idx + 1;
            }
        }

        // // Read the file 8 bits at a time
        // for (idx, byte) in buf.as_bytes().into_iter().enumerate() {
        //     // Convert to char unsafely to avoid slow validity checks
        //     // let c: char = unsafe { char::from_u32_unchecked((*byte).into()) };

        //     // Read the file until the next boundary
        //     if is_word_boundary(c) {
        //         // There might be tokens between us and the previous boundary
        //         if last_idx != idx {
        //             // Get the str between both indices
        //             let word: &str = unsafe { buf.get_unchecked(last_idx..idx) };

        //             let mut word_token = Token::new_word(last_idx + total_bytes, word);

        //             if let TokenType::Identifier = word_token.ty {
        //                 let symbol = self.idents.get_or_intern(word);
        //                 word_token.symbol = Some(symbol);
        //             }

        //             tokens.push(word_token);
        //         }

        //         // Store the boundary token
        //         let token = Token::new_word(idx + total_bytes, c);
        //         tokens.push(token);

        //         last_idx = idx + 1;
        //     }
        // }

        tokens
    }
}

impl Default for AsciiLexer {
    fn default() -> Self {
        Self::new()
    }
}

// pub trait Lexer<'a> {
//     fn tokenize<T: std::io::Read>(&mut self, reader: &mut BufReader<T>) -> Vec<Token<'a>>;
//
//     fn tokenize_from_reader<T: std::io::Read>(
//         &mut self,
//         reader: &mut BufReader<T>,
//     ) -> Vec<Token<'a>> {
//         return self.tokenize(reader);
//     }
// }

fn is_word_boundary(word: char) -> bool {
    matches!(
        word,
        '\n' | ' '
            | '"'
            | '#'
            | '%'
            | '&'
            | '\''
            | '('
            | ')'
            | '*'
            | '+'
            | ','
            | '-'
            | '.'
            | '/'
            | ':'
            | ';'
            | '<'
            | '='
            | '>'
            | '?'
            | '`'
            | '{'
            | '}'
    )
}

// fn is_word_boundary(word: char) -> bool {
//     matches!( word,'\n' | ' ' | '"' | '\'' | '(' | ')' | '*' | '+' | ',' | '-' | '.' | '/' | ':' | ';' | '<' | '=' | '>' | '`' | '{' | '}')
// }

// fn is_token_boundary(word: char) -> bool {
//     matches!(word, '"' | '\'' | '(' | ')' | '*' | '+' | ',' | '-' | '.' | '/' | ':' | ';' | '<' | '=' | '>' | '`' | '{' | '}')
// }
//
// // Match whitespace boundaries
// fn is_whitespace(word: char) -> bool {
//     matches!(word, '\n' | ' ')
// }
//
// // Match bracket boundaries
// fn is_paren(word: char) -> bool {
//     matches!(word, '(' | ')')
// }
// fn is_curly(word: char) -> bool {
//     matches!(word, '{' | '}')
// }
// fn is_square(word: char) -> bool {
//     matches!(word, '[' | ']')
// }
// fn is_angle(word: char) -> bool {
//     matches!(word, '<' | '>')
// }
//
// // Match quote boundaries
// fn is_single_quote(word: char) -> bool {
//     matches!(word, '\'')
// }
// fn is_double_quote(word: char) -> bool {
//     matches!(word, '"')
// }
// fn is_backtick(word: char) -> bool {
//     matches!(word, '`')
// }
//
// // Match operator boundaries
// fn is_operator(word: char) -> bool {
//     matches!(word, '+' | '-' | '/' | '*')
// }
//
// // Match colon boundaries
// fn is_colon(word: char) -> bool {
//     matches!(word, ':')
// }
// fn is_semicolon(word: char) -> bool {
//     matches!(word, ';')
// }
//
// // Other special chars
// fn is_period(word: char) -> bool {
//     matches!(word, '.')
// }
// fn is_equals(word: char) -> bool {
//     matches!(word, '=')
// }
