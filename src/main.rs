#![feature(iter_map_windows)]

use std::fs::File;
use std::error::Error;
use std::io::BufReader;

use llvm_compiler::{lexer::{AsciiLexer, Lexer}, parse::parser::Parser, token::Token, READER_CAPACITY};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/80_char_lines.src").unwrap();
    let mut reader = BufReader::with_capacity(READER_CAPACITY, &file);

    let tokens = AsciiLexer::new().tokenize_from_reader(&mut reader);

    println!("{}", tokens.len());

    // println!("{:?}", &tokens);
    // let token = &tokens[1];
    // println!("Last: {token:?}, get_string: {}", token.get_string(&tokens, &mut reader));

    // let print_token = |token: &Token, reader: &mut BufReader<&File>| { print!("{}", token.get_string(&tokens, reader)) };
    // tokens.iter().for_each(|token| { print_token(token, &mut reader) });

    // println!();

    // let parsed = match Parser::new(&mut reader, &tokens).parse(&tokens) {
    //     Ok(result) => result,
    //     Err(parse_err) => { panic!("{parse_err}") }
    // };

    // let mut reader = BufReader::with_capacity(READER_CAPACITY, &file);
    // parsed.iter().map(|idx| &tokens[*idx]).for_each(|token| { print_token(token, &mut reader) });

    Ok(())
}