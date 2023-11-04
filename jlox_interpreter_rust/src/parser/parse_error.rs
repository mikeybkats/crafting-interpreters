use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    message: String,
    // token: Token,
}

impl ParseError {
    pub fn new(message: &String) -> Self {
        // println!("parse error: {}", message);
        Self {
            message: message.clone(),
            // token: token.clone(),
        }
    }

    pub fn _get_error(&self) -> &String {
        &self.message
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {
    // You can add more functionality here if necessary.
    // For simple errors, often nothing more is needed.
}
