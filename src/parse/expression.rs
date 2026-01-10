use crate::parse::parser::Parser;
use crate::parsed_to_str;
use crate::token::Token;
use log::trace;

pub(super) fn expression<'p, 't>(parser: &'p mut Parser<'t>) -> miette::Result<()> {
    // Write from the stack until pattern is matched, discarding the matched variant
    macro_rules! write_until {
        (None) => {
            while let Some(popped) = parser.pop() {
                match popped {
                    _ => {
                        parser.write(popped);
                    }
                }
            }
        };
        ($($variant:path)|+) => {
            while let Some(popped) = parser.pop() {
                match popped {
                    $($variant(_))|* => {
                        break;
                    }
                    _ => {
                        parser.write(popped);
                    }
                }
            }
        };
    }

    // Get last precedence from the stack
    macro_rules! last_precedence {
        () => {
            parser.last().map_or(0, |token| token.precedence().1)
        };
    }

    // let mut last_token = Token::Unknown(Inner::new(0, ""));
    let mut last_rp = 0; // Right precedence of the operator just to the left of our current position

    loop {
        let token = parser.next_token();

        if let None = token {
            trace!("Found None");
            return Ok(());
        }

        let token = token.unwrap();
        trace!("Found {}", &token);
        match token {
            // Write idents/literals immediately
            Token::Identifier(_) | Token::NumericLiteral(_) => {
                parser.write(token);
            }

            // Handle end of expression
            Token::SemiColon(_) => {
                trace!("End of expression");

                write_until!(None);
                parser.write(token);
                return Ok(());
            }

            // If the incoming token is an intial, write it and push it to the stack
            Token::OpenBracket(_) | Token::OpenCurly(_) => {
                let (_, rp) = token.precedence();

                parser.write(token);

                parser.push(token);
                last_rp = rp;
            }

            // If the incoming token is a terminal, pop and write the stack until you come across an initial token with small enough precedence
            Token::CloseBracket(_) | Token::CloseCurly(_) => {
                write_until!(Token::OpenBracket | Token::OpenCurly);

                last_rp = last_precedence!();

                parser.write(token);
            }

            _ => {
                let (lp, rp) = token.precedence();
                if lp < last_rp {
                    // If the incoming operator has lower lp than the last rp on the stack, pop and write until this is no longer the case
                    trace!("Popping until lp < popped_rp");
                    while let Some(popped) = parser.pop() {
                        let (_pop_lp, pop_rp) = popped.precedence();
                        trace!("Popped {}, with ({}, {})", &popped, _pop_lp, pop_rp);

                        parser.write(popped);
                        if lp < pop_rp {
                            break;
                        }
                    }
                }
                // If the incoming operator has higher lp than the last rp on top of the stack, push
                parser.push(token);
                last_rp = rp;
            }
        }

        trace!(
            "LRP: {}, Stack: {}, Parsed: {}",
            last_rp,
            parsed_to_str(&parser.stack),
            parsed_to_str(&parser.parsed)
        );
    }
}
