// #![feature(iter_map_windows)]

use std::fs::File;
use std::error::Error;
use std::io::BufReader;

use llvm_compiler::{lexer::{AsciiLexer, Lexer}, READER_CAPACITY, token::Token, parse::parser::Parser};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/example.src").unwrap();
    let mut reader = BufReader::with_capacity(READER_CAPACITY, file);

    let mut lexer = AsciiLexer::default();
    let tokens = lexer.tokenize_from_reader(&mut reader);

    println!("{}", tokens.len());

    // println!("{:?}", &tokens);
    // let token = &tokens[1];
    // println!("Last: {token:?}, get_string: {}", token.get_string(&tokens, &mut reader));

    let print_token = |token: &Token, reader: &mut BufReader<File>| { print!("{:?}", token); };
    tokens.iter().for_each(|token| { print_token(token, &mut reader); });

    println!();
    println!();
    println!("Parsed:");

    let mut parser = Parser::from(lexer);
    let parsed_tokens = parser.parse(&tokens, &mut reader);
    parsed_tokens.iter().for_each(|token| { print_token(token, &mut reader); });

    // let parsed = match Parser::new(&mut reader, &tokens).parse(&tokens) {
    //     Ok(result) => result,
    //     Err(parse_err) => { panic!("{parse_err}") }
    // };

    // let mut reader = BufReader::with_capacity(READER_CAPACITY, &file);
    // parsed.iter().map(|idx| &tokens[*idx]).for_each(|token| { print_token(token, &mut reader) });

    Ok(())
}
