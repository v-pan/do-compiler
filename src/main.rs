mod lexer;
pub mod token;
pub mod token_err;

use std::fs::File;
use std::error::Error;
use std::io::BufReader;

use lexer::Lexer;
use packed_struct::PackedStruct;

use crate::token::Token;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./example.src").unwrap();
    let mut reader = BufReader::new(&file);

    let mut lexer = Lexer::new();
    let mut tokens = vec![];

    lexer.tokenize(&mut tokens, &mut reader);

    println!("{:?}", &tokens);
    let last = tokens.last().unwrap();
    println!("{last:?}");
    println!("{:?} {}", last, Token::unpack(last).unwrap().get_string(&tokens, &mut reader));

    Ok(())
}
