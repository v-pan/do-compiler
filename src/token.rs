pub enum Token {
    CommentStart,
    FunctionStart,
}

pub struct TokenParser {}

impl TokenParser {
    pub fn new() -> Self {
        TokenParser {}
    }

    pub fn parse_token(&self, token: &str) -> Option<Token> {
        match token {
            "//" => Some(Token::CommentStart),
            "fun" => Some(Token::FunctionStart),
            _ => None
        }
    }
}
