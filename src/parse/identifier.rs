use crate::Span;
use super::{SyntaxError, Parse};

use std::io::{BufReader, Read};
use std::fs::File;
use std::str::{from_utf8, from_utf8_unchecked};

const INVALID_CHARS: [char; 7] = ['(', ')', '{', '}', ';', ':', ','];

#[derive(Clone, Debug)]
pub struct Identifier {
    pub ident: String,
    pub span: Span
}
impl Identifier {
    pub fn new(ident: &str, mut span: Span) -> Self {
        if ident.contains(&INVALID_CHARS) {
            panic!("Invalid identifier {}", ident);
        }

        span.size = ident.len();
        Identifier { ident: ident.to_string(), span }
    }
}
impl Parse for Identifier {
    fn parse(reader: &mut BufReader<&File>, span: Span) -> Result<Self, SyntaxError> {
        let mut ident = String::new();
        let mut bytes_read = 0;

        // TODO: Move into its own iterator?
        let mut invalid_idx = 0;
        let mut buf: [u8; 1] = [0; 1];
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

            let mut unread = || {
                bytes_read -= 1;
                reader.seek_relative(-1).unwrap();
            };

            // Stop reading at expected endpoints
            match cur_char {
                "(" => { unread(); break },
                "{" => { unread(); break },
                " " => { unread(); break },
                _ => {},
            }

            ident += cur_char;
        }

        Ok(Identifier {
            ident,
            span: Span::new(span.start, bytes_read)
        })
    }
}
impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident)
    }
}
