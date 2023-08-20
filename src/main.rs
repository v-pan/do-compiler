mod line;
mod token;
mod node;
mod parse;
mod lexer;
mod span;

// use lexer::TokenLexer;
use parse::Function;
// use parse::Node;
use parse::Parse;
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

    pub fn parse(&self) -> Result<(), parse::SyntaxError> {
        let mut buf_reader = BufReader::new(&self.file_handle);

        // Read until we reach a global keyword. All global keywords are expected to be followed by
        // a space.
        let mut buf = vec![];
        let mut last_loc = Span::default();
        loop {
            buf_reader.read_until(b' ', &mut buf).unwrap();

            let keyword = from_utf8(&buf).unwrap().trim();
            println!("Got keyword {keyword}");

            match keyword {
                "fun" => {
                    // Move last_loc up
                    let kw_len = "fun".len();
                    last_loc.end = buf.len() - kw_len;

                    // Read in the function signature
                    let mut sig_buf = vec![];
                    buf_reader.read_until(b'{', &mut sig_buf).expect("Expected a function body");

                    let mut sig_span = Span::after(last_loc);
                    sig_span.extend(kw_len);

                    let sig_str = from_utf8(&sig_buf).unwrap().trim();
                    let function_node = Function::parse(sig_str, sig_span)?;

                    println!("Parsed function: {}", function_node);

                    break;
                }
                "//" => {
                    buf.clear();
                    buf_reader.read_until(b'\n', &mut vec![]).unwrap();
                },
                _ => {
                    panic!("Not a valid keyword")
                }
            }
        }

        Ok(())
    }
}

use std::error::Error;
use std::str::from_utf8;
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("example.src").unwrap();

    let parser = FileParser::new(file);
    parser.parse()?;

    Ok(())
}
