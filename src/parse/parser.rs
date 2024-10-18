use miette::bail;

use crate::token::Token;

pub struct Parser<'a> {
    index: usize,
    parsed: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(index: usize, parsed: Vec<Token<'a>>) -> Self {
        Parser { index, parsed }
    }

    fn peek(&self, tokens: &'a [Token<'a>]) -> Token<'a> {
        let mut index = self.index;

        loop {
            let next_token = tokens[index];
            index += 1;

            if let Token::Space | Token::Newline = next_token {
                continue;
            } else {
                return next_token;
            }
        }
    }

    fn next(&mut self, tokens: &'a [Token<'a>]) -> Token<'a> {
        loop {
            let next_token = tokens[self.index];
            self.index += 1;

            if let Token::Space | Token::Newline = next_token {
                continue;
            } else {
                return next_token;
            }
        }
    }

    pub fn parse(mut self, tokens: &'a [Token<'a>]) -> miette::Result<Vec<Token<'_>>> {
        let mut stack: Vec<Token<'_>> = vec![];

        let token = self.peek(tokens);

        match token {
            Token::Identifier(_) => {
                self.parse_expression(tokens, &mut stack)?;
            }
            _ => {}
        }

        Ok(self.parsed)
    }

    fn parse_expression(
        &mut self,
        tokens: &'a [Token<'a>],
        stack: &mut Vec<Token<'a>>,
    ) -> miette::Result<()> {
        let lhs = self.next(tokens);

        stack.push(lhs);

        loop {
            let operator = self.next(tokens);

            stack.push(operator);

            match operator {
                Token::Plus(_) => self.parse_binary_operator(tokens, stack),
                Token::SemiColon(_) => {
                    break;
                }
                _ => {
                    bail!("Expected operator, found {:?}", operator);
                }
            }
        }

        self.parsed.extend(stack.iter());
        stack.clear();

        Ok(())
    }

    fn parse_binary_operator(&mut self, tokens: &'a [Token<'a>], stack: &mut Vec<Token<'a>>) {
        let op = stack.pop().unwrap();

        let rhs = self.next(tokens);

        stack.push(rhs);
        stack.push(op);
    }
}

impl<'a> Default for Parser<'a> {
    fn default() -> Self {
        Self::new(0, vec![])
    }
}
