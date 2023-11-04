use crate::{
    lox::Lox,
    scanner::{
        expr::Expr,
        token::{self, Literal, Token, TokenType},
    },
};

use super::parse_error::ParseError;

pub struct Parser<'a> {
    current: usize,
    tokens: &'a Vec<Token>,
    lox: Box<&'a Lox>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>, lox: &'a Lox) -> Self {
        Self {
            current: 0,
            tokens,
            lox: Box::new(lox),
        }
    }

    /// # parse
    ///
    /// The main function of the Parser implementation. This starts the process of parsing an expression.
    ///
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        return self.expression();
    }

    /// # expression
    ///
    /// Compiles the expression.
    ///
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    /// # equality
    ///
    /// the equality function returns the binary expression to make comparisons between a left and right expression
    ///
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.comparison()?;

        while self.match_symbol(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.get_operator()?.clone();

            let right: Expr = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// # match_symbol
    ///
    /// Loops through a given vector of tokens, advances the cursor and returns true if any of the tokens in the vector match the next token.
    ///
    fn match_symbol(&mut self, tokens: &[TokenType]) -> bool {
        for token_type in tokens.iter() {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// # check
    ///
    /// Returns true if the next token is of the given type. Returns false if the parser has reached the end of the file.
    ///
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        match self.peek() {
            Some(token) => token.token_type == *token_type,
            None => false,
        }
    }

    /// # advance
    ///
    /// Advances the cursor (consuming the current token), and returns the current Token as an Option<&Token>
    ///
    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) => token.token_type == TokenType::Eof,
            None => false,
        }
    }

    /// # peek
    ///
    /// returns the next token.
    ///
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// # previous
    ///
    /// returns the previous token.
    ///
    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn get_operator(&self) -> Result<&Token, ParseError> {
        self.previous()
            .ok_or_else(|| ParseError::new(&String::from("Expected Operator")))
    }

    /// # comparison
    ///
    /// defines an equality comparison when encountering: >, >=, <, <=
    ///
    /// _rule_:
    ///
    /// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    ///
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_symbol(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token = self.get_operator()?.clone();

            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// # term
    ///
    /// compiles the term function when encountering a minus or plus symbole. This defines a term from a left and right expression. Notice how the rule starts with the factor expression.
    ///
    /// _rule:_
    ///
    /// term           → factor ( ( "-" | "+" ) factor )* ;
    ///
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_symbol(&[TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.get_operator()?.clone();

            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    /// # factor
    ///
    /// compiles multiplication and division when current token is a slash or asteriscks
    ///
    /// _rule_:
    ///
    /// factor         → unary ( ( "/" | "*" ) unary )* ;
    ///
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_symbol(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.get_operator()?.clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// # unary
    ///
    /// compiles unary expression when encoundering a bang or minus symbol
    ///
    /// _rule:_  
    ///
    /// unary          → ( "!" | "-" ) unary | primary ;
    ///
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_symbol(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.get_operator()?.clone();

            let right = self.unary()?;

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    /// # Primary
    ///
    /// Primary is the highest level of precedence. This rule processes literal strings and numbers as well as booleans and all expressions:
    ///
    /// _rule_:
    ///
    /// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    ///
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_symbol(&[TokenType::False]) {
            return Ok(Expr::Literal {
                value: token::Literal::Bool(false),
            });
        }

        if self.match_symbol(&[TokenType::True]) {
            return Ok(Expr::Literal {
                value: token::Literal::Bool(true),
            });
        }

        if self.match_symbol(&[TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: token::Literal::Nil,
            });
        }

        if self.match_symbol(&[TokenType::Number, TokenType::String]) {
            let prev_literal;

            match self.previous() {
                Some(token) => {
                    prev_literal = token.literal.clone();
                }
                None => prev_literal = Literal::None,
            }

            return Ok(Expr::Literal {
                value: prev_literal,
            });
        }

        if self.match_symbol(&[TokenType::LeftParen]) {
            let expr = self.expression()?;

            self.consume(
                TokenType::RightParen,
                String::from("Expect ')' after expression."),
            )?;

            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(self.error(self.peek().unwrap(), "Expected expression.".to_string()))
    }

    /// # consume
    ///
    /// checks to see if the next token is of the expected type. If so, it consumes the token. Else, it returns a parse error.
    ///
    fn consume(&mut self, token_type: TokenType, message: String) -> Result<&Token, ParseError> {
        match self.check(&token_type) {
            true => {
                return Ok(self.advance().unwrap());
            }
            false => match self.peek() {
                Some(token) => Err(self.error(token, message)),
                None => return Err(ParseError::new(&"Token not found".to_string())),
            },
        }
    }

    /// # error
    ///
    /// creates a new parse error
    ///
    /// The error() method returns the error instead of throwing it because we want to let the calling method inside the parser decide whether to unwind or not.
    ///
    fn error(&self, token: &Token, message: String) -> ParseError {
        self.lox.as_ref().error(token, &message);
        ParseError::new(&message)
    }

    /// # synchronize
    ///
    /// Catches exceptions at statement boundaries, and brings the parser to the correct state. This prevents unwanted error messages from polluting the user's dev experience.
    ///
    fn _synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().unwrap().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().unwrap().token_type {
                TokenType::Class => {}
                TokenType::Fun => {}
                TokenType::Var => {}
                TokenType::For => {}
                TokenType::If => {}
                TokenType::While => {}
                TokenType::Print => {}
                TokenType::Return => (),
                _ => {
                    self.advance();
                }
            }
        }

        self.advance();
    }
}
