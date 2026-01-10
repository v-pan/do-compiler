use crate::parse::parser::Parser;
use crate::parse::state::{ExpressionState, ParseState};
use crate::parsed_to_str;
use crate::token::Token;
use log::trace;

pub(super) fn expression<'p, 't>(parser: &'p mut Parser<'t>, last_precedence: u8) -> () {
    // Write from the stack until pattern is matched, discarding the matched variant
    macro_rules! write_until {
        (None) => {
            while let Some(popped) = parser.pop_token() {
                match popped {
                    _ => {
                        parser.write(popped);
                    }
                }
            }
        };
        ($($variant:path)|+) => {
            while let Some(popped) = parser.pop_token() {
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

    // let token = parser.next_token();

    // if let None = token {
    //     trace!("Found None");
    //     return;
    // }

    // let token = token.unwrap();
    // trace!("Found {}", &token);

    loop {
        let state = parser.peek_state();
        let initiator = match state {
            ParseState::Expression(inner) => inner.initiator,
            _ => panic!("Expected expression state"),
        };

        trace!(
            "stack: {}, parsed: {}",
            parsed_to_str(&parser.stack),
            parsed_to_str(&parser.parsed)
        );
        let token = parser.next_token();
        let token = match token {
            None => {
                write_until!(None);
                break;
            }
            Some(token) => {
                if token.terminates(initiator) {
                    trace!("End of expression, popping until {}", &initiator);
                    while let Some(popped) = parser.pop_token() {
                        if popped == initiator {
                            break;
                        }
                        parser.write(popped);
                    }
                    parser.write(token);
                    break;
                }

                match token {
                    Token::OpenBracket(_) | Token::VariableDeclaration(_) => {
                        parser.push(token);
                        parser.write(token);

                        parser.push_state(ExpressionState::new(token).into());
                        become expression(parser, 0);
                    }
                    Token::Identifier(_) | Token::NumericLiteral(_) => {
                        trace!("Found non-operator {}", &token);

                        parser.write(token);
                        continue;
                    }
                    _ => token,
                }
            }
        };

        let (lp, rp) = token.precedence();
        trace!("Found operator {token} ({lp}, {rp})");
        if lp < last_precedence {
            trace!("{lp} < {last_precedence}, popping stack");

            // If the incoming operator has lower lp than the last rp on the stack, pop and write until this is no longer the case
            while let Some(popped) = parser.pop_token() {
                let (_pop_lp, pop_rp) = popped.precedence();
                trace!("    popped {}, with ({}, {})", &popped, _pop_lp, pop_rp);

                if popped != initiator {
                    parser.write(popped)
                };
                if lp >= pop_rp {
                    break;
                }
            }

            parser.push(token);
            continue;
        }

        parser.push(token);
        become expression(parser, rp);
    }
    parser.pop_state();
}
