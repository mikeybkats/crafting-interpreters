use crate::scanner::token::TokenType;
use std::fmt;

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
    token_type: TokenType,
}

impl RuntimeError {
    pub fn new(message: String, token_type: TokenType) -> Self {
        Self {
            message,
            token_type,
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.message, self.token_type)
    }
}

impl std::error::Error for RuntimeError {}
