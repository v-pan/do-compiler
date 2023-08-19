mod line;
mod token;
mod node;
mod parse;
mod lexer;
mod span;

use lexer::TokenLexer;
use span::Span;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct FileParser {
    file_handle: File
}

impl FileParser {
    pub fn new(file_handle: File) -> Self {
        FileParser { file_handle }
    }

    pub fn parse(&self) -> Result<(), std::io::Error> {
        let buf_reader = BufReader::new(&self.file_handle);

        for line in buf_reader.lines() {
            let span = Span::default();
            let token_lexer = TokenLexer::new(line?, span);

            for token in token_lexer {
                match token {
                    parse::Node::Fun(fun) => {
                        println!("Function: {fun}");
                    }
                    _ => { break; }
                }
            }
        }

        Ok(())
    }
}

use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("example.src").unwrap();

    let parser = FileParser::new(file);
    parser.parse()?;

    Ok(())
}
