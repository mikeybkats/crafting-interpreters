use crate::scanner::token::Literal;
use std::fmt;

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
    literal: Literal,
}

impl RuntimeError {
    pub fn new(message: String, literal: Literal) -> Self {
        Self {
            message,
            literal: literal.clone(),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.message, self.literal)
    }
}

impl std::error::Error for RuntimeError {}
