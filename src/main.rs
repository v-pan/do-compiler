mod parse;
mod span;

use parse::Keyword;
use parse::Parse;
use span::Span;

use std::fs::File;
use std::io::BufReader;

struct FileParser {
    file: File
}

impl FileParser {
    pub fn new(file: File) -> Self {
        FileParser { file }
    }

    pub fn parse(&mut self) -> Result<(), parse::SyntaxError> {
        let mut buf_reader = BufReader::new(&self.file);

        // Expect:
        //  - Include
        //  - Function definition
        //  - Struct definition
        //  - Class definition
        //  - Variable definition
        //  - Value definition
        //  - Expression

        let span = Span::new(0, 0);
        let keyword = Keyword::parse(&mut buf_reader, span)?;
        match keyword {
            Keyword::Function(function) => {
                println!("Got function {function:#?}");
            }
        }

        Ok(())
    }
}

use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("example.src").unwrap();

    let mut parser = FileParser::new(file);
    parser.parse()?;

    Ok(())
}
