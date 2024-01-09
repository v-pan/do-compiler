use std::io::{BufReader, Read};
use crate::token::Token;

pub enum LexicalContext {
    FunctionDecl,
    DocumentRoot,
}

pub struct AsciiLexer {}

impl AsciiLexer {
    pub fn new() -> AsciiLexer {
        AsciiLexer {}
    }
}

impl Lexer for AsciiLexer {
    fn tokenize<T: std::io::Read>(&mut self, tokens: &mut Vec<Token>, reader: &mut BufReader<T>) {
        let mut total_bytes = 0;
        let mut buf = String::new();

        loop {
            // Clear buffer from last iteration
            buf.clear();

            // Read in as much as we can at once
            let bytes_read = reader.read_to_string(&mut buf).unwrap();
            if bytes_read == 0 { break; } // We finished tokenizing this reader
            tokens.reserve(bytes_read / 2); // Speculatively allocate space in the buffer for tokens. Chosen arbitrarily
            
            let mut last_idx = 0;
            
            // Read the file 8 bits at a time
            for (idx, byte) in buf.as_bytes().into_iter().enumerate() {
                // Convert to char unsafely to avoid slow validity checks
                let c: char = unsafe { char::from_u32_unchecked((*byte).into()) };

                // Read the file until the next boundary 
                if is_word_boundary(c) {
                    // There might be tokens between us and the previous boundary 
                    if last_idx != idx {
                        // Get the str between both indices
                        let word = unsafe { buf.get_unchecked(last_idx..idx) };
                        let word_token = Token::new_word(last_idx + total_bytes, word);

                        tokens.push(word_token);
                    }
                    let token = Token::new_char(idx + total_bytes, c);
                    tokens.push(token);
                    
                    last_idx = idx+1;
                }
            }

            total_bytes += bytes_read;
        }
    }
}

pub trait Lexer {
    fn tokenize<T: std::io::Read>(&mut self, tokens: &mut Vec<Token>, reader: &mut BufReader<T>);

    fn tokenize_from_reader<T: std::io::Read>(&mut self, reader: &mut BufReader<T>) -> Vec<Token> {
        let mut tokens = vec![];
    
        self.tokenize(&mut tokens, reader);
    
        tokens
    }
}

// fn is_word_boundary(word: char) -> bool {
//     matches!( word,'\n' | ' ' | '"' | '#' | '%' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | '-' | '.' | '/' | ':' | ';' | '<' | '=' | '>' | '?' | '`' | '{' | '}')
// }

fn is_word_boundary(word: char) -> bool {
    matches!( word,'\n' | ' ' | '"' | '\'' | '(' | ')' | '*' | '+' | ',' | '-' | '.' | '/' | ':' | ';' | '<' | '=' | '>' | '`' | '{' | '}')
}

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
