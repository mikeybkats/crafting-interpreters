use std::fmt;

use crate::ast_grammar::token::Token;

#[derive(Debug, Clone)]
pub struct ParseError {
    message: String,
    token: Token,
}

impl ParseError {
    pub fn new(message: &str, token: &Token) -> Self {
        Self {
            message: String::from(message),
            token: token.clone(),
        }
    }

    pub fn get_error(&self) -> (&str, &Token) {
        (&self.message, &self.token)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.message, self.token)
    }
}

impl std::error::Error for ParseError {
    // You can add more functionality here if necessary.
    // For simple errors, often nothing more is needed.
}
