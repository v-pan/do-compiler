mod line;
mod token;
use line::lexer::LineLexer;

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
        let mut buf_reader = BufReader::new(&self.file_handle);
        let token_parser = token::TokenParser::new();

        'line: loop {
            let mut contents = String::new();
            let res = buf_reader.read_line(&mut contents)?;

            if res == 0 {
                break Ok(())
            }

            let mut lexer = LineLexer::new(&contents);

            'token: loop {
                if let Some(s) = lexer.next() {
                    let cur_token = token_parser.parse_token(s);

                    use token::Token;
                    if let Some(t) = cur_token {
                        match t {
                            Token::CommentStart => continue 'line,
                            Token::FunctionDef => FileParser::parse_function_def(&mut lexer),
                        }
                    }
                } else {
                    break 'token;
                }

            }
        }
    }

    fn parse_function_def(lexer: &mut LineLexer) {

    }
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("example.src").unwrap();

    let parser = FileParser::new(file);
    parser.parse()?;

    Ok(())
}
