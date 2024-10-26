use log::{debug, trace};
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

    fn peek(&self, tokens: &'a [Token<'a>]) -> Option<Token<'a>> {
        let mut index = self.index;

        loop {
            if index >= tokens.len() - 1 {
                return None;
            }

            let next_token = tokens[index];
            index += 1;

            if let Token::Space | Token::Newline = next_token {
                continue;
            } else {
                return Some(next_token);
            }
        }
    }

    fn next(&mut self, tokens: &'a [Token<'a>]) -> Option<Token<'a>> {
        loop {
            if self.index >= tokens.len() - 1 {
                return None;
            }

            let next_token = tokens[self.index];
            self.index += 1;

            if let Token::Space | Token::Newline = next_token {
                continue;
            } else {
                return Some(next_token);
            }
        }
    }

    pub fn parse(mut self, tokens: &'a [Token<'a>]) -> miette::Result<Vec<Token<'_>>> {
        let mut stack: Vec<Token<'_>> = vec![];

        loop {
            let token = self.peek(tokens);

            if token.is_none() {
                break;
            }

            let token = token.unwrap();

            match token {
                Token::Identifier(_) => {
                    self.parse_expression(tokens, &mut stack)?;
                }
                Token::FunctionDeclaration(_) => {
                    self.parse_function_declaration(tokens, &mut stack)?;
                }
                _ => {
                    bail!("Unexpected token {:?}", token);
                }
            }
        }

        Ok(stack)
    }

    fn consume(&mut self, tokens: &'a [Token<'a>]) {
        loop {
            let next_token = tokens[self.index];
            self.index += 1;

            if let Token::Space | Token::Newline = next_token {
                continue;
            } else {
                break;
            }
        }
    }

    fn parse_function_declaration(
        &mut self,
        tokens: &'a [Token<'a>],
        stack: &mut Vec<Token<'a>>,
    ) -> miette::Result<()> {
        trace!("Parsing function declaration");
        let func = self.next(tokens).unwrap();

        let ident = self.next(tokens);
        if ident.is_none() {
            bail!("Expected function identifier after {:?}", func);
        }
        let ident = ident.unwrap();
        trace!("Found identifier {:?}", &ident);

        let open_bracket = self.next(tokens);
        if open_bracket.is_none() {
            bail!("Expected open bracket after {:?}", ident);
        }
        let open_bracket = open_bracket.unwrap();
        stack.push(open_bracket);

        // Parse parameters
        self.parse_expression(tokens, stack)?;

        // Parse return type
        self.parse_expression(tokens, stack)?;

        let curly_or_equals = stack.last();

        match curly_or_equals {
            Some(Token::OpenCurly(_)) => {
                stack.push(*curly_or_equals.unwrap());

                // TODO: Parse function body
                self.parse_expression(tokens, stack)?;

                let close_curly = self.next(tokens);

                match close_curly {
                    Some(Token::CloseCurly(_)) => {
                        stack.push(close_curly.unwrap());
                    }
                    None => {
                        bail!("Expected end of function body after {:?}", stack.last());
                    }
                    _ => {
                        bail!("Expected }} found {:?}", close_curly.unwrap());
                    }
                }
            }
            Some(Token::Equals(_)) => {
                stack.push(*curly_or_equals.unwrap());
                self.parse_expression(tokens, stack)?;
            }
            None => {
                let found = *curly_or_equals.unwrap();
                bail!(
                    "Expected function body or expression after {:?}, found {:?}",
                    stack.pop().unwrap(),
                    found,
                );
            }
            _ => {
                let found = *curly_or_equals.unwrap();
                bail!(
                    "Expected {{ or = after {:?}, found {:?}",
                    stack.pop().unwrap(),
                    found,
                );
            }
        }

        Ok(())
    }

    fn parse_expression(
        &mut self,
        tokens: &'a [Token<'a>],
        stack: &mut Vec<Token<'a>>,
    ) -> miette::Result<()> {
        self.parse_expression_inner(tokens, stack, 0)?;
        let terminator = self.next(tokens).unwrap();

        stack.push(terminator);

        Ok(())
    }

    fn parse_expression_inner(
        &mut self,
        tokens: &'a [Token<'a>],
        stack: &mut Vec<Token<'a>>,
        precedence: u8,
    ) -> miette::Result<()> {
        trace!("Parsing expression, stack: {:?}", &stack);

        loop {
            let token = self.peek(tokens);

            if token.is_some() {
                trace!("Found token {:?}, index: {}", &token.unwrap(), self.index);
            }

            match token {
                // Arguments
                Some(Token::Identifier(_)) => {
                    trace!("Token is identifier");

                    stack.push(token.unwrap());
                    self.consume(tokens);

                    trace!("Stack: {:?}", stack);
                }

                // Operators
                Some(Token::Plus(_))
                | Some(Token::Comma(_))
                | Some(Token::Colon(_))
                | Some(Token::Arrow(_)) => {
                    trace!("Token is operator");

                    let operator = token.unwrap();
                    let (lhs_precedence, rhs_precedence) = operator.precedence();

                    trace!(
                        "lhs_precedence: {}, rhs_precedence: {}, precedence: {}",
                        lhs_precedence,
                        rhs_precedence,
                        precedence
                    );

                    if lhs_precedence < precedence {
                        break;
                    }

                    self.consume(tokens);

                    // Want to find the next operator with higher precedence
                    self.parse_expression_inner(tokens, stack, rhs_precedence)?;
                    stack.push(operator);
                }

                // End of expression
                Some(Token::SemiColon(_))
                | Some(Token::CloseBracket(_))
                | Some(Token::OpenCurly(_)) => {
                    break;
                }

                None => {
                    bail!("Unexpected end of tokens. Did you forget to close the expression");
                }
                _ => {
                    let token = token.unwrap();
                    bail!(
                        "Expected argument, operator, or end of expression. Found {:?}",
                        &token
                    );
                }
            }
        }

        trace!("Parsed expression, stack: {:?}", stack);

        Ok(())
    }
}

impl<'a> Default for Parser<'a> {
    fn default() -> Self {
        Self::new(0, vec![])
    }
}
