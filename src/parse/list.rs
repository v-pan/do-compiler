use crate::Span;
use super::{SyntaxError, Parse};

use std::io::{BufReader, Read};
use std::fs::File;
use std::str::{from_utf8_unchecked, from_utf8};

#[derive(Debug)]
pub struct List {
    pub items: Vec<String>,
    pub span: Span
}
impl Parse for List {
    fn parse(reader: &mut BufReader<&File>, span: Span) -> Result<Self, SyntaxError> {
        let mut bytes_read = 0;
        let mut items = Vec::new();

        // TODO: Move into its own iterator?
        let mut invalid_idx = 0;
        let mut buf: [u8; 1] = [0; 1];

        let mut cur_item = String::new();
        let mut depth = 0;
        loop {
            bytes_read += reader.read(&mut buf[invalid_idx..]).unwrap();
            invalid_idx = 0;

            // Read the next valid char
            let cur_char = match from_utf8(&buf) {
                Ok(c) => c,
                Err(err) => {
                    invalid_idx = err.valid_up_to();
                    if err.error_len().is_none() {
                        panic!("Reached invalid char")
                    }
                    unsafe { from_utf8_unchecked(&buf[..invalid_idx]) };
                    todo!("Change buf to hold 4 bytes to handle variable length UTF8 chars")
                }
            };

            match cur_char {
                "(" | "{" | "<" => { depth += 1 },
                ")" | "}" | ">" => {
                    // We reached a closing bracket outside of any list items, i.e. the end of the list
                    if depth == 0 {
                        items.push(cur_item);
                        break;
                    } else {
                        depth -= 1;
                    }
                },
                "," => {
                    if depth == 0 {
                        items.push(cur_item);
                        cur_item = String::new();
                        continue;
                    }
                }
                _ => {},
            }

            cur_item += cur_char;
        }

        Ok(List {
            items,
            span: Span::new(span.start, bytes_read)
        })
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items_string = self.items.join(", ");
        write!(f, "{items_string}")
    }
}

