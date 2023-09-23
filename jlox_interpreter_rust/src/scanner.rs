use std::{cell::RefCell, rc::Rc};

use crate::{
    error::ErrorReporter,
    token::{Token, TokenType},
};

pub struct Scanner {
    source: String,
    source_length: usize,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    error_reporter: Rc<RefCell<ErrorReporter>>,
}
impl Scanner {
    pub fn new(source: String, error_reporter: Rc<RefCell<ErrorReporter>>) -> Self {
        let source_length = source.chars().count().clone();

        Self {
            source,
            source_length,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error_reporter,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line,
        ));

        &self.tokens
    }

    fn increment_current(&mut self) {
        self.current += 1;
    }

    fn scan_token(&mut self) {
        let c: Option<char> = self.advance();

        println!("character: {}", c.unwrap());

        match c {
            Some(c) => match c {
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

                '!' => {
                    self.add_token(if self.match_char('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    });
                    self.increment_current();
                }

                '=' => {
                    self.add_token(if self.match_char('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    });
                    self.increment_current();
                }

                '<' => {
                    self.add_token(if self.match_char('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    });
                    self.increment_current();
                }

                '>' => {
                    self.add_token(if self.match_char('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    });
                    self.increment_current();
                }

                ' ' => (),
                '\r' => (),
                '\t' => (),
                '\n' => self.line += 1,

                '/' => {
                    if self.match_char('/') {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash)
                    }
                }

                _ => {
                    self.error_reporter
                        .borrow_mut()
                        .report_error_message(self.line, "Unexpected Character");
                }
            },
            None => {
                self.error_reporter
                    .borrow_mut()
                    .report_error_message(self.line, "No Character");
            }
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

    fn advance(&mut self) -> Option<char> {
        self.current += 1;

        let character = self.source.chars().nth(self.current);

        println!("SOURCE CHAR: {}", character.unwrap());

        match character {
            Some(character) => Some(character),
            _ => None,
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.current_char()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source_length
    }

    fn current_char(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    fn match_char(&self, expected_char: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.current_char() != expected_char {
            return false;
        }

        true
    }
}
