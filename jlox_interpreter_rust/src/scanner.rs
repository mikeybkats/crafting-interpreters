use std::{cell::RefCell, rc::Rc};

use crate::{
    error::ErrorReporter,
    token::{StringOrNumber, Token, TokenType},
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
                    let mut token_type = TokenType::Bang;
                    if self.match_char('=') {
                        self.increment_current();
                        token_type = TokenType::BangEqual;
                    }
                    self.add_token(token_type);
                }

                '=' => {
                    let mut token_type = TokenType::Equal;
                    if self.match_char('=') {
                        self.increment_current();
                        token_type = TokenType::EqualEqual
                    }
                    self.add_token(token_type);
                }

                '<' => {
                    let mut token_type = TokenType::Less;
                    if self.match_char('=') {
                        self.increment_current();
                        token_type = TokenType::LessEqual
                    }
                    self.add_token(token_type);
                }

                '>' => {
                    let mut token_type = TokenType::Greater;
                    if self.match_char('=') {
                        self.increment_current();
                        token_type = TokenType::GreaterEqual
                    }
                    self.add_token(token_type);
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

                '"' => self.string(),

                _ => {
                    if c.is_numeric() {
                        self.number()
                    } else {
                        self.error_reporter
                            .borrow_mut()
                            .report_error_message(self.line, "Unexpected Character");
                    }
                }
            },
            None => {
                // self.error_reporter
                //     .borrow_mut()
                //     .report_error_message(self.line, "No Character");
                // println!("End of input");
                // Do nothing
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_value(token_type, None)
    }

    fn add_token_with_value(&mut self, token_type: TokenType, literal: Option<StringOrNumber>) {
        let lexeme = self.source[self.start..self.current].to_string();

        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);

        self.current += 1;

        match c {
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

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source_length {
            return Some('\0');
        }

        self.source.chars().nth(self.current + 1)
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

    fn string(&mut self) {
        // while the next character is not a double quote and it's not EOF
        while self.peek() != '"' && !self.is_at_end() {
            // if there is a new line tally line index
            if self.peek() == '\n' {
                self.line += 1;
            }
            // go to next char
            self.advance();
        }

        if self.is_at_end() {
            self.error_reporter
                .borrow_mut()
                .report_error_message(self.line, "Unterminated string.");
            return;
        }

        // Get the The closing " quotation mark
        self.advance();

        // Trim the surrounding quotes.
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_with_value(
            TokenType::String,
            Some(StringOrNumber::Str(value.to_string())),
        );
    }

    fn number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        let next_is_numeric = self
            .peek_next()
            .map_or(false, |character| character.is_numeric());

        if self.peek() == '.' && next_is_numeric {
            self.advance();
        }

        let number: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token_with_value(TokenType::Number, Some(StringOrNumber::Num(number)))
    }
}
