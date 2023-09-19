use std::{io::{BufReader, BufRead}, fs::File};

use packed_struct::PackedStruct;
use unicode_segmentation::UnicodeSegmentation;

use crate::token::Token;

// Make sure this list is sorted for binary search.
// Can be done by calling sort() in another file.
// Benchmarks suggest this is faster than using a HashSet.
const WORD_BOUNDARIES: [char; 24] = ['\n', ' ', '"', '#', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '`', '{', '}'];

pub struct UnicodeLexer {}

impl UnicodeLexer {
    pub fn new() -> UnicodeLexer {
        UnicodeLexer {  }
    }
}

impl Lexer for UnicodeLexer {
    fn tokenize<T: std::io::Read>(&mut self, tokens: &mut Vec<Token>, reader: &mut BufReader<T>) {
        let mut total_bytes = 0;
        let mut buf = String::new();
        
        loop {
            buf.clear();
            let bytes_read = reader.read_line(&mut buf).unwrap();

            if bytes_read == 0 {
                break;
            }

            let filter = buf.split_word_bound_indices().filter_map(|(idx, word)| {
                let idx = (idx + total_bytes).try_into().unwrap();

                Token::new(idx, word).into()
            });

            tokens.extend(filter);

            total_bytes += bytes_read;
        }
    }

    fn tokenize_packed<T: std::io::Read>(&mut self, tokens: &mut Vec<[u8; 5]>, reader: &mut BufReader<T>) {
        let mut total_bytes = 0;
        let mut buf = String::new();
        
        loop {
            buf.clear();
            let bytes_read = reader.read_line(&mut buf).unwrap();

            if bytes_read == 0 {
                break;
            }

            let filter = buf.split_word_bound_indices().filter_map(|(idx, word)| {
                let idx = (idx + total_bytes).try_into().unwrap();

                Token::new(idx, word).pack().ok()
            });

            tokens.extend(filter);

            total_bytes += bytes_read;
        }
    }
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
            buf.clear();
            let bytes_read = reader.read_line(&mut buf).unwrap();

            if bytes_read == 0 {
                break;
            }

            let mut last_idx = 0;

            // Split on boundary characters, push the resulting tokens to vec
            buf.match_indices(|c| {
                WORD_BOUNDARIES.binary_search(&c).is_ok()
            }).for_each(|(idx_in_line, sep)| {
                if last_idx == idx_in_line { return; }

                let word = &buf[last_idx..idx_in_line];

                let idx = (last_idx + total_bytes).try_into().unwrap();
                let word_token = Token::new(idx, word);

                let idx = (idx_in_line + total_bytes).try_into().unwrap();
                let sep_token = Token::new(idx, sep);

                last_idx = (idx_in_line + sep.len()).try_into().unwrap();

                tokens.push(word_token);
                tokens.push(sep_token);
            });

            total_bytes += bytes_read;
        }
    }

    fn tokenize_packed<T: std::io::Read>(&mut self, tokens: &mut Vec<[u8; 5]>, reader: &mut BufReader<T>) {
        let mut total_bytes = 0;
        let mut buf = String::new();
        
        loop {
            buf.clear();
            let bytes_read = reader.read_line(&mut buf).unwrap();

            if bytes_read == 0 {
                break;
            }

            let mut last_idx = 0;

            let filter = buf
            .match_indices(|c| { WORD_BOUNDARIES.binary_search(&c).is_ok() })
            .flat_map(|(idx, sep)| {
                let word = &buf[last_idx..idx];

                let values = [(last_idx, word), (idx, sep)];
                
                last_idx = idx + sep.len();
                
                values
            })
            .filter_map(|(idx, word)| {
                let idx = (idx + total_bytes).try_into().unwrap();

                Token::new(idx, word).pack().ok()
            });

            tokens.extend(filter);

            total_bytes += bytes_read;
        }
    }
}

pub trait Lexer {
    fn tokenize<T: std::io::Read>(&mut self, tokens: &mut Vec<Token>, reader: &mut BufReader<T>);

    fn tokenize_packed<T: std::io::Read>(&mut self, tokens: &mut Vec<[u8; 5]>, reader: &mut BufReader<T>);

    fn tokenize_from_reader(&mut self, reader: &mut BufReader<&File>) -> Vec<Token> {
        let mut tokens = vec![];
    
        self.tokenize(&mut tokens, reader);
    
        tokens
    }
    
    fn tokenize_from_reader_packed(&mut self, reader: &mut BufReader<&File>) -> Vec<[u8; 5]> {
        let mut tokens = vec![];
    
        self.tokenize_packed(&mut tokens, reader);
    
        tokens
    }
}