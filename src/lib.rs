pub mod lexer;
pub mod parse;
pub mod token;

pub fn parsed_to_str<I, D>(iterable: I) -> String
where
    I: IntoIterator<Item = D>,
    D: std::fmt::Display,
{
    let mut iterator = iterable.into_iter();

    let head = match iterator.next() {
        None => return String::from("[]"),
        Some(x) => format!("{}", x),
    };
    let body = iterator.fold(head, |a, v| format!("{}{}", a, v));
    format!("{}", body)
}

#[cfg(test)]
mod tests {
    use log::trace;

    use crate::{lexer::AsciiLexer, parse::parser::Parser, parsed_to_str, token::Token};

    fn init_logging() {
        let _ = pretty_env_logger::env_logger::builder()
            .is_test(true)
            .try_init();
    }

    #[test]
    fn plus_is_operator() {
        let token = Token::from(0, "+");
        assert!(token.is_operator());
    }

    #[test]
    fn lexer_handle_buffer() {
        let mut lexer = AsciiLexer::new();
        let buf = String::from("1 + 2;");

        let tokens = lexer.tokenize(&buf);

        let target = [
            Token::from(0, "1"),
            Token::from(2, "+"),
            Token::from(4, "2"),
            Token::from(5, ";"),
        ];

        let matching = tokens
            .iter()
            .zip(&target)
            .all(|(a, b)| a.as_str() == b.as_str());

        assert!(matching);
    }

    #[test]
    fn lexer_handle_unknowns() {
        let mut lexer = AsciiLexer::new();
        let buf = String::from("1 + test;");

        let tokens = lexer.tokenize(&buf);

        let target = [
            Token::from(0, "1"),
            Token::from(2, "+"),
            Token::from(4, "test"),
            Token::from(8, ";"),
        ];

        assert!(matches!(tokens[0], Token::NumericLiteral(_)));
        assert!(matches!(tokens[2], Token::Identifier(_)));

        let matching = tokens
            .iter()
            .zip(&target)
            .all(|(a, b)| a.as_str() == b.as_str());

        assert!(matching);
    }

    #[test]
    fn parser_addition() {
        init_logging();

        trace!("Hello from test");

        let mut lexer = AsciiLexer::new();
        let buf = String::from("1 + 2;");

        let tokens: Vec<Token<'_>> = lexer.tokenize(&buf);

        let parser = Parser::new(0, &tokens);
        let parsed = parser.parse().unwrap();

        let target = [tokens[0], tokens[2], tokens[1], tokens[3]];

        let matching = parsed
            .iter()
            .zip(&target)
            .all(|(a, b)| a.as_str() == b.as_str());

        assert!(matching);
    }

    #[test]
    fn parser_nested_addition() {
        init_logging();

        let mut lexer = AsciiLexer::new();
        let buf = String::from("1 + (2 + 3);");

        let tokens: Vec<Token<'_>> = lexer.tokenize(&buf);

        let parser = Parser::new(0, &tokens);
        let parsed = parser.parse().unwrap();

        trace!("{}", parsed_to_str(&parsed));

        let target_buf = String::from("1 ( 2 3 + ) + ;");
        let target = lexer.tokenize(&target_buf);

        let matching = parsed
            .iter()
            .zip(&target)
            .all(|(a, b)| a.as_str() == b.as_str());
        assert!(matching);
        assert!(tokens.len() == target.len());
    }

    #[test]
    fn parser_addition_before_multiplication() {
        init_logging();

        let mut lexer = AsciiLexer::new();
        let buf = String::from("1 * 2 + 3;");

        let tokens: Vec<Token<'_>> = lexer.tokenize(&buf);

        let parser = Parser::new(0, &tokens);
        let parsed = parser.parse().unwrap();

        trace!("Final parsed tokens:");
        trace!("{}", parsed_to_str(&parsed));

        let target_buf = String::from("1 2 * 3 +;");
        let target = lexer.tokenize(&target_buf);

        let matching = parsed
            .iter()
            .zip(&target)
            .all(|(a, b)| a.as_str() == b.as_str());
        assert!(matching);
        assert!(tokens.len() == target.len());
    }

    #[test]
    fn parser_addition_after_multiplication() {
        init_logging();

        let mut lexer = AsciiLexer::new();
        let buf = String::from("1 + 2 * 3;");

        let tokens: Vec<Token<'_>> = lexer.tokenize(&buf);

        let parser = Parser::new(0, &tokens);
        let parsed = parser.parse().unwrap();

        trace!("Final parsed tokens:");
        trace!("{}", parsed_to_str(&parsed));

        let target_buf = String::from("1 2 3 * +;");
        let target = lexer.tokenize(&target_buf);

        let matching = parsed
            .iter()
            .zip(&target)
            .all(|(a, b)| a.as_str() == b.as_str());
        assert!(matching);
        assert!(tokens.len() == target.len());
    }

    #[test]
    fn parser_mixed_addition_multiplication_parentheses() {
        init_logging();

        let mut lexer = AsciiLexer::new();
        let buf = String::from("A * (B + C * D) + E;");

        let tokens: Vec<Token<'_>> = lexer.tokenize(&buf);

        let parser = Parser::new(0, &tokens);
        let parsed = parser.parse().unwrap();

        trace!("Final parsed tokens:");
        trace!("{}", parsed_to_str(&parsed));

        let target_buf = String::from("A ( B C D * + ) * E +;");
        let target = lexer.tokenize(&target_buf);

        let matching = parsed
            .iter()
            .zip(&target)
            .all(|(a, b)| a.as_str() == b.as_str());
        assert!(matching);
        assert!(tokens.len() == target.len());
    }
}
