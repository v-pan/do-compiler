#![feature(iter_map_windows)]

use std::fs::File;
use std::error::Error;
use std::io::BufReader;

use llvm_compiler::lexer::{AsciiLexer, Lexer};

const READER_CAPACITY: usize = 500_000_000; // 500mb

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/example.src").unwrap();
    let mut reader = BufReader::with_capacity(READER_CAPACITY, &file);

    let tokens = AsciiLexer::new().tokenize_from_reader(&mut reader);

    // println!("{:?}", &tokens);
    // let token = &tokens[1];
    // println!("Last: {token:?}, get_string: {}", token.get_string(&tokens, &mut reader));
    tokens.iter().for_each(|token| print!("{}", token.get_string(&tokens, &mut reader)));

    Ok(())
}