use crate::scanner::token::Token;
use std::fmt;

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
    token: Token,
}

impl RuntimeError {
    pub fn new(message: String, token: &Token) -> Self {
        Self {
            message,
            token: token.clone(),
        }
    }

    pub fn get_error(&self) -> (&str, &Token) {
        (self.message.as_str(), &self.token)
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.message, self.token.token_type)
    }
}

impl std::error::Error for RuntimeError {}
