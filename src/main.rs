mod lexer;
pub mod token;

use std::fs::File;
use std::error::Error;
use std::io::BufReader;

use lexer::Lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./example.src").unwrap();
    let mut reader = BufReader::new(&file);

    let mut lexer = Lexer::new();
    let mut tokens = vec![];

    lexer.tokenize(&mut tokens, &mut reader);

    println!("{:?}", &tokens);

    Ok(())
}
