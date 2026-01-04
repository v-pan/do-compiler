// #![feature(iter_map_windows)]
mod print;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use llvm_compiler::parse::parser::Parser;
use llvm_compiler::{lexer::AsciiLexer, token::Token};

pub const READER_CAPACITY: usize = 100_000_000;

fn main() -> miette::Result<()> {
    pretty_env_logger::init();

    let file = File::open("./examples/infix.src").unwrap();
    let mut reader = BufReader::with_capacity(READER_CAPACITY, file);

    let mut lexer = AsciiLexer::new();
    let mut buf = String::new();

    // Read in as much as we can at once
    let bytes_read = reader.read_to_string(&mut buf).unwrap(); // TODO: There is some edge
                                                               // case behaviour here when
                                                               // a file is too long to
                                                               // store in memory. This is
                                                               // currently unhandled.

    let tokens: Vec<Token<'_>> = lexer.tokenize(&buf);

    for token in tokens.iter() {
        print!("{:?}", token);
    }

    println!("{}", tokens.len());

    println!();

    let mut parser = Parser::new(0, &tokens);

    let source = buf.clone();
    // let parsed = parse(&mut parser).map_err(|report| report.with_source_code(source));

    // println!("{:?}", parsed);

    // print::print_parsed_tree(&parsed.unwrap());

    Ok(())
}
