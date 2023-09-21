pub struct IdentifierInfo {
    pub token_idx: usize,
}

impl IdentifierInfo {
    pub fn new(token_idx: usize, string_delegate: Box<dyn FnOnce() -> String>) -> Self {
        Self {
            token_idx,
        }
    }
}