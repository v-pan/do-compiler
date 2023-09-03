use crate::Span;
use super::{SyntaxError, Parse};
use super::identifier::Identifier;

use std::io::{BufReader, Read};
use std::fs::File;
use std::str::{from_utf8, from_utf8_unchecked};

#[derive(Clone, Debug)]
pub enum Type {
    Explicit(Identifier, Span),
    Inferred,
}
impl Type {
    pub fn from(value: &str, span: Span) -> Self {
        if value == "" {
            return Type::Inferred;
        }

        let identifier = Identifier::new(&value.trim(), span);
        let span = identifier.span;

        Type::Explicit(identifier, span)
    }
}
impl Parse for Type {
    fn parse(reader: &mut BufReader<&File>, span: Span) -> Result<Self, SyntaxError> {
        let mut ty = String::new();
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
                "{" => { unread(); break },
                _ => {},
            }

            ty += cur_char;
        }

        Ok(Type::Explicit(Identifier::new(ty.trim(), Span::after(span)), Span::new(span.start, bytes_read)))
    }
}
