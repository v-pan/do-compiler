#![feature(iter_map_windows)]

use std::fs::File;
use std::error::Error;
use std::io::BufReader;

use llvm_compiler::{lexer::{AsciiLexer, Lexer}, parse::parser::Parser, token::Token};

const READER_CAPACITY: usize = 500_000_000; // 500mb

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/infix.src").unwrap();
    let mut reader = BufReader::with_capacity(READER_CAPACITY, &file);

    let tokens = AsciiLexer::new().tokenize_from_reader(&mut reader);

    println!("{:?}", &tokens);
    // let token = &tokens[1];
    // println!("Last: {token:?}, get_string: {}", token.get_string(&tokens, &mut reader));

    let print_token = |token: &Token, reader: &mut BufReader<&File>| { print!("{}", token.get_string(&tokens, reader)) };
    tokens.iter().for_each(|token| { print_token(token, &mut reader) });

    println!();

    let parsed = Parser::parse(&tokens);
    parsed.iter().map(|idx| &tokens[*idx]).for_each(|token| { print_token(token, &mut reader) });

    Ok(())
}