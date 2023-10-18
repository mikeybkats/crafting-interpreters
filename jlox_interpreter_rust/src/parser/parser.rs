use crate::scanner::{
    expr::Expr,
    token::{self, Literal, Token, TokenType},
};

use super::parse_error::ParseError;

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    /// # equality
    ///
    /// the equality function returns the binary expression to make comparisons between a left and right expression
    ///
    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.match_symbol(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token;
            match self.previous() {
                Some(token_operator) => operator = token_operator.clone(),
                None => {
                    operator = Token::new(TokenType::Eof, "".to_string(), token::Literal::None, 1)
                }
            }
            let right: Expr = self.comparison();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn match_symbol(&mut self, tokens: Vec<TokenType>) -> bool {
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
    /// returns true if the token is of the given type
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

    /// advance
    ///
    /// advances the cursor (consuming the current token), and returns the current Token as an Option<&Token>
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
    /// returns the next token as an Option<&Token>
    ///
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    /// # comparison
    ///
    /// defines an equality comparison when encountering: >, >=, <, <=
    ///
    /// _rule_:
    ///
    /// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    ///
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_symbol(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token;
            match self.previous() {
                Some(token) => {
                    operator = token.clone();
                    let right = self.term();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    }
                }
                None => {
                    // Token not found, throw an error
                }
            }
        }

        expr
    }

    /// # term
    ///
    /// compiles the term function when encountering a minus or plus symbole. This defines a term from a left and right expression. Notice how the rule starts with the factor expression.
    ///
    /// _rule:_
    ///
    /// term           → factor ( ( "-" | "+" ) factor )* ;
    ///
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_symbol(vec![TokenType::Minus, TokenType::Plus]) {
            let operator: Token;
            match self.previous() {
                Some(token) => {
                    operator = token.clone();
                    let right = self.term();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    }
                }
                None => {
                    // Token not found, throw an error
                }
            }
        }

        expr
    }

    /// # factor
    ///
    /// compiles multiplication and division when current token is a slash or asteriscks
    ///
    /// _rule_:
    ///
    /// factor         → unary ( ( "/" | "*" ) unary )* ;
    ///
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_symbol(vec![TokenType::Slash, TokenType::Star]) {
            match self.previous() {
                Some(token) => {
                    let operator = token.clone();
                    let right = self.term();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    }
                }
                None => {
                    // Token not found, throw an error
                }
            }
        }

        expr
    }

    /// # unary
    ///
    /// compiles unary expression when encoundering a bang or minus symbol
    ///
    /// _rule:_  
    ///
    /// unary          → ( "!" | "-" ) unary | primary ;
    ///
    fn unary(&mut self) -> Expr {
        if self.match_symbol(vec![TokenType::Bang, TokenType::Minus]) {
            match self.previous() {
                Some(token) => {
                    let operator = token.clone();

                    let right = self.unary();

                    return Expr::Unary {
                        operator,
                        right: Box::new(right),
                    };
                }
                None => {
                    // token not found, throw error
                }
            }
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
    fn primary(&mut self) -> Expr {
        if self.match_symbol(vec![TokenType::False]) {
            return Expr::Literal {
                value: token::Literal::Bool(false),
            };
        }

        if self.match_symbol(vec![TokenType::True]) {
            return Expr::Literal {
                value: token::Literal::Bool(true),
            };
        }

        if self.match_symbol(vec![TokenType::Nil]) {
            return Expr::Literal {
                value: token::Literal::Nil,
            };
        }

        if self.match_symbol(vec![TokenType::Number, TokenType::String]) {
            let prev_literal;

            match self.previous() {
                Some(token) => {
                    prev_literal = token.literal.clone();
                }
                None => prev_literal = Literal::None,
            }

            return Expr::Literal {
                value: prev_literal,
            };
        }

        if self.match_symbol(vec![TokenType::LeftParen]) {
            let expr = self.expression();

            self.consume(
                TokenType::RightParen,
                String::from("Expect ')' after expression."),
            );

            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }

        // TODO: Rewrite to throw an error here. This will require rewriting much of the application. Errors should bubble up to the top of the application
        Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Literal::None,
            }),
        }
    }

    // private Token consume(TokenType type, String message) {
    //     if (check(type)) return advance();

    //     throw error(peek(), message);
    //   }
    /// # consume
    ///
    /// checks to see if the next token is of the expected type. If so, it consumes the token. Else, it returns a parse error.
    fn consume(&mut self, token_type: TokenType, message: String) -> Result<&Token, ParseError> {
        if self.check(&token_type) {
            match self.advance() {
                Some(token) => return Ok(token),
                None => return Err(ParseError::new()),
            }
        }

        match self.peek() {
            Some(token) => Err(self.error(token, message)),
            None => Err(ParseError::new()),
        }
    }

    // private ParseError error(Token token, String message) {
    //     Lox.error(token, message);
    //     return new ParseError();
    //   }
    /// # error
    ///
    /// creates a new parse error
    fn error(&self, token: &Token, message: String) -> ParseError {
        ParseError::new()
    }
}
