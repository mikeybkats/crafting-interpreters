use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    source_length: usize,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        let source_length = source.chars().count().clone();
        Self {
            source,
            source_length,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            _ => (), // Do nothing for other characters, or handle them as needed
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_by_type(token_type, None)
    }

    fn add_token_by_type(&mut self, token_type: TokenType, literal: Option<String>) {
        let lexeme = self.source[self.start..self.current].to_string();

        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }

    fn advance(&self) -> char {
        'a'
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source_length
    }
}
