use std::io::{BufReader, BufRead};

use packed_struct::PackedStruct;
use unicode_segmentation::UnicodeSegmentation;

use crate::token::{Token, TokenType};

pub struct Lexer {
    
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {  }
    }

    pub fn tokenize<T: std::io::Read>(&mut self, tokens: &mut Vec<[u8; 8]>, reader: &mut BufReader<T>) {
        let mut line = 0;
        let mut buf = String::new();
        
        loop {
            buf.clear();
            let bytes_read = reader.read_line(&mut buf).unwrap();

            if bytes_read == 0 {
                break;
            }

            tokens.extend(
                buf.split_word_bound_indices().filter_map(|(idx, word)| {
                    let idx = idx.try_into().unwrap();

                    Token::try_keyword_packed(idx, word)
                    .or(
                        Token::try_paren_packed(idx, word)
                    ).or(
                        Token::try_operator_packed(idx, word)
                    ).or(
                        Token::try_seperator_packed(idx, word)
                    ).or(
                        Token::try_whitespace_packed(idx, word)
                    ).or(
                        Token::try_quote_packed(idx, word)
                    ).or(
                        Token::new(idx, TokenType::Unknown).pack()
                    ).ok()
                })
            );

            line += 1;
        }
    }
}