#![allow(dead_code)]

use std::hash::{Hash, Hasher};

use crate::environment::generate_id;

use super::object::Object;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Object.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
    pub _id: String,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Object>,
        line: usize,
        _id: String,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
            _id: generate_id(),
        }
    }

    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
            && self.lexeme == other.lexeme
            && self.line == other.line
            && self._id == other._id
    }
}

impl Eq for Token {}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lexeme.hash(state);
        self.line.hash(state);
        self._id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_equality() {
        let token1 = Token {
            token_type: TokenType::Identifier,
            lexeme: String::from("test"),
            literal: Option::None,
            line: 1,
            _id: generate_id(),
        };

        let token2 = token1.clone();

        assert_eq!(token1, token2);
    }

    #[test]
    fn test_token_inequality() {
        let token1 = Token {
            token_type: TokenType::Identifier,
            lexeme: String::from("test"),
            literal: Option::None,
            line: 1,
            _id: generate_id(),
        };

        let token2 = Token {
            token_type: TokenType::Identifier,
            lexeme: String::from("test"),
            literal: Option::None,
            line: 2,
            _id: generate_id(),
        };

        assert_ne!(token1, token2);
    }
}
