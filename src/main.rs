// #![feature(iter_map_windows)]

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use llvm_compiler::parse::parser::Parser;
use llvm_compiler::{lexer::AsciiLexer, token::Token, READER_CAPACITY};

fn main() -> miette::Result<()> {
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

    let tokens: Vec<Token<'_>> = lexer.tokenize(&mut buf);

    for token in tokens.iter() {
        print!("{:?}", token);
    }

    println!("{}", tokens.len());

    println!();

    let parser = Parser::default();

    let parsed = parser.parse(&tokens)?;

    println!("{:?}", parsed);

    Ok(())
}
