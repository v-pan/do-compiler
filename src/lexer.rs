use std::{io::{BufReader, Read}, fs::File};
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
            tokens.reserve(bytes_read / 2); // Try to preallocate enough space for tokens. Chosen arbitrarily

            if bytes_read == 0 {
                break;
            }

            let mut last_idx = 0;

            // Split on boundary characters, push the resulting tokens to vec
            buf.match_indices(|c| {
                is_word_boundary(c)
            }).for_each(|(idx_in_buf, sep)| {
                if idx_in_buf != last_idx {
                    let word = unsafe { buf.get_unchecked(last_idx..idx_in_buf) };

                    let idx = (last_idx + total_bytes).try_into().unwrap();
                    let word_token = Token::new(idx, word);

                    tokens.push(word_token);
                }

                let idx = (idx_in_buf + total_bytes).try_into().unwrap();
                let sep_token = Token::new(idx, sep);

                last_idx = idx_in_buf + 1;

                tokens.push(sep_token);
            });

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