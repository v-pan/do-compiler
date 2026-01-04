use crate::token::{Inner, Token};

pub struct AsciiLexer;

impl<'a> AsciiLexer {
    pub fn new() -> Self {
        AsciiLexer
    }

    pub fn tokenize(&mut self, buf: &'a String) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        let mut last_token = Token::Unknown(Inner::new(0, ""));
        let mut last_idx = 0;

        for (idx, byte) in buf.as_bytes().iter().enumerate() {
            // Convert to char unsafely to avoid slow validity checks
            let c: char = unsafe { char::from_u32_unchecked((*byte).into()) };

            if is_word_boundary(c) {
                // There might be tokens between us and the previous boundary
                if last_idx != idx {
                    // Get the str between both indices
                    let word: &str = unsafe { buf.get_unchecked(last_idx..idx) };

                    let word_token = Token::from(last_idx, word);

                    tokens.push(word_token);
                }

                let word: &str = unsafe { buf.get_unchecked(idx..idx + 1) };

                // Store the boundary token
                let token = Token::from(idx, word);

                // Look behind to see if this is a two character boundary token
                match token {
                    Token::GreaterThan(_) => {
                        if let Token::Minus(_) = last_token {
                            tokens.pop();
                            let token = Token::from(idx - 1, "->");

                            last_token = token;
                            tokens.push(token);
                        }
                    }
                    Token::Equals(_) => {
                        if let Token::Equals(_) = last_token {
                            // TODO: Comparison operators
                        }
                        last_token = token;
                        tokens.push(token);
                    }
                    _ => {
                        last_token = token;
                        tokens.push(token);
                    }
                }

                last_idx = idx + 1;
            }
        }

        tokens
    }
}

impl Default for AsciiLexer {
    fn default() -> Self {
        Self::new()
    }
}

fn is_word_boundary(word: char) -> bool {
    matches!(
        word,
        '\n' | ' '
            | '"'
            | '#'
            | '%'
            | '&'
            | '\''
            | '('
            | ')'
            | '*'
            | '+'
            | ','
            | '-'
            | '.'
            | '/'
            | ':'
            | ';'
            | '<'
            | '='
            | '>'
            | '?'
            | '`'
            | '{'
            | '}'
    )
}
