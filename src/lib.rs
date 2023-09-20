#![feature(test)]

extern crate test;
pub mod lexer;
pub mod token;
pub mod error;
pub mod parse;

#[allow(dead_code)]
const READER_CAPACITY: usize = 100_000_000;

#[cfg(test)]
mod tests {
    use std::{fs::File, io::{BufReader, Seek}};

    use test::{Bencher, black_box};

    use crate::{lexer::{AsciiLexer, Lexer}, READER_CAPACITY};

    #[bench]
    fn bench_ascii_tokenize(b: &mut Bencher) {
        let mut lexer = AsciiLexer::new();
        let file = File::open("./examples/example.src").unwrap();
        let mut reader = BufReader::with_capacity(READER_CAPACITY, &file);
        
        b.iter(|| {
            reader.rewind().unwrap();
            let tokens = black_box(lexer.tokenize_from_reader(&mut reader));
            tokens
        });
    }

    #[bench]
    fn bench_token_clone(b: &mut Bencher) {
        let mut lexer = AsciiLexer::new();
        let file = File::open("./examples/example.src").unwrap();
        let mut reader = BufReader::with_capacity(READER_CAPACITY, &file);
        let tokens = lexer.tokenize_from_reader(&mut reader);

        b.iter(|| {
            tokens.clone()
        });
    }

    #[bench]
    fn bench_ascii_tokenize_80_char_lines(b: &mut Bencher) {
        let mut lexer = AsciiLexer::new();
        let file = File::open("./examples/80_char_lines.src").unwrap();
        let mut reader = BufReader::with_capacity(READER_CAPACITY, &file);
        
        b.iter(|| {
            reader.rewind().unwrap();
            let tokens = black_box(lexer.tokenize_from_reader(&mut reader));
            tokens
        });
    }
    
    #[bench]
    fn bench_token_get_string(b: &mut Bencher) {
        let mut lexer = AsciiLexer::new();
        let file = File::open("./examples/long_identifier.src").unwrap();
        let mut reader = BufReader::with_capacity(READER_CAPACITY, &file);

        let tokens = black_box(lexer.tokenize_from_reader(&mut reader));
        let last = tokens.last().unwrap();

        b.iter(|| { black_box(last.get_string(&tokens, &mut reader)) } );
    }

    // #[bench]
    // fn bench_unicode_tokenize(b: &mut Bencher) {
    //     let mut lexer = UnicodeLexer::new();
    //     let file = File::open("./examples/example.src").unwrap();
    //     let mut reader = BufReader::with_capacity(READER_CAPACITY, &file);
        
    //     b.iter(|| {
    //         reader.rewind().unwrap();
    //         let tokens = black_box(lexer.tokenize_from_reader(&mut reader));
    //         tokens
    //     });
    // }

    // #[bench]
    // fn bench_ascii_tokenize_packed(b: &mut Bencher) {
    //     let mut lexer = AsciiLexer::new();
    //     let file = File::open("./examples/example.src").unwrap();
    //     let mut reader = BufReader::with_capacity(READER_CAPACITY, &file);
        
    //     b.iter(|| {
    //         reader.rewind().unwrap();
    //         let tokens = black_box(lexer.tokenize_from_reader_packed(&mut reader));
    //         tokens
    //     });
    // }

}