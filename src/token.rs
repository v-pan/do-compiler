pub enum Token {
    CommentStart,
    FunctionDef,
}

pub struct TokenParser {}

impl TokenParser {
    pub fn new() -> Self {
        TokenParser {}
    }

    pub fn parse_token(&self, token: &str) -> Option<Token> {
        match token {
            "//" => Some(Token::CommentStart),
            "fun" => Some(Token::FunctionDef),
            _ => None
        }
    }
}
