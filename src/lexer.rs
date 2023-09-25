use std::{io::{BufReader, Read}, fs::File, str::from_utf8_unchecked};
use crate::token::Token;

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
            buf.clear();
            let bytes_read = reader.read_to_string(&mut buf).unwrap();

            if bytes_read == 0 { break; }

            tokens.reserve(bytes_read / 2); // Try to preallocate enough space for tokens. Chosen arbitrarily

            let mut last_idx = 0;

            for (idx, byte) in buf.as_bytes().into_iter().enumerate() {
                let slice = &[*byte];
                let c = unsafe { from_utf8_unchecked(slice) };
                if is_word_boundary(unsafe { char::from_u32_unchecked((*byte).into()) }) {
                    if last_idx != idx {
                        let word = unsafe { buf.get_unchecked(last_idx..idx) };
                        let word_token = Token::new(idx + total_bytes + bytes_read, word);

                        tokens.push(word_token);
                    }

                    let sep_token = Token::new(idx + total_bytes + bytes_read, c );
                    last_idx = idx + 1;
                    tokens.push(sep_token);
                }
            }

            total_bytes += bytes_read;
        }
    }
}

pub trait Lexer {
    fn tokenize<T: std::io::Read>(&mut self, tokens: &mut Vec<Token>, reader: &mut BufReader<T>);

    fn tokenize_from_reader(&mut self, reader: &mut BufReader<File>) -> Vec<Token> {
        let mut tokens = vec![];
    
        self.tokenize(&mut tokens, reader);
    
        tokens
    }
}

fn is_word_boundary(word: char) -> bool {
    matches!( word,'\n' | ' ' | '"' | '#' | '%' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | '-' | '.' | '/' | ':' | ';' | '<' | '=' | '>' | '?' | '`' | '{' | '}')
}