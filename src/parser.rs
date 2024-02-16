use crate::ast_grammar::stmt::Stmt;
use crate::ast_grammar::token::{self, Token, TokenType};
use crate::error::parse_error::ParseError;

use crate::ast_grammar::expr::Expr;

pub struct Parser<'a> {
    current: usize,
    tokens: &'a Vec<Token>,
    empty_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            current: 0,
            tokens,
            empty_token: Token::new(TokenType::Nil, "".to_string(), Some(token::Literal::Nil), 0),
        }
    }

    /// # parse
    ///
    /// The main function of the Parser implementation. This starts the process of parsing an expression.
    ///
    // pub fn parse(&mut self) -> Result<Vec<Expr>, ParseError> {
    //     return self.expressions();
    // }
    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    /// # expressions
    ///
    /// Compiles list of expressions
    ///
    /// expressions --> expression (, expressions)*;
    ///
    fn _expressions(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut expressions = Vec::new();

        if let Ok(mut expression) = self.equality() {
            loop {
                expressions.push(expression);

                if !self.match_symbol(&[TokenType::Comma]) {
                    break;
                }
                match self.equality() {
                    Ok(new_expression) => expression = new_expression,
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(expressions)
    }

    /// # expression
    ///
    /// Compiles the expression.
    ///
    fn expression(&mut self) -> Result<Expr, ParseError> {
        // self.equality()
        self.assignment()
    }

    /// # declaration
    /// Called repeatedly when parsing statments in either block or script mode
    /// This is the where the application should synchronize when the parser panics.
    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_symbol(&[TokenType::Var]) {
            match self.var_declaration() {
                Ok(stmt) => Ok(stmt),
                Err(e) => {
                    self.synchronize();
                    Err(e)
                }
            }
        } else {
            match self.statement() {
                Ok(stmt) => Ok(stmt),
                Err(e) => {
                    self.synchronize();
                    Err(e)
                }
            }
        }
    }

    /// # statement
    /// "A program is a list of statements, and we parse one of those statements using this method"
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_symbol(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_symbol(&[TokenType::LeftBrace]) {
            return self.block();
        }

        self.expression_statement()
    }

    /// # print_statement
    /// parse a print statement
    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;

        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Print {
            expression: Box::new(value),
        })
    }

    /// # var_declaration
    /// parse a variable declaration
    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name;

        match self.consume(TokenType::Identifier, "Expected variable name.") {
            Ok(token) => name = token.clone(),
            Err(e) => return Err(e),
        }

        let mut initializer: Option<Expr> = None;

        if self.match_symbol(&[TokenType::Equal]) {
            let expr = self.expression()?;
            initializer = Some(expr);
        }

        // consume twice and advance the cursor
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var {
            name: name.clone(),
            initializer: Box::new(initializer.unwrap()),
        })
    }

    /// # expression_statement
    /// parse an expression
    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;

        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;

        Ok(Stmt::Expression {
            expression: Box::new(value),
        })
    }

    fn block(&mut self) -> Result<Stmt, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(Stmt::Block { statements })
    }

    /// # assignment
    /// parse an assignment. Will fail if the variable does not exist.
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.equality()?;

        if self.match_symbol(&[TokenType::Equal]) {
            let equals = self.previous().unwrap().clone();
            let value = self.assignment()?;

            match expr {
                Expr::Variable { name } => {
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    });
                }
                _ => {
                    return Err(ParseError::new(
                        &"Invalid assignment target.".to_string(),
                        &equals,
                    ));
                }
            }
        }

        Ok(expr)
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
        match self.peek() {
            Some(token) if !self.is_at_end() => token.token_type == *token_type,
            _ => false,
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
            .ok_or_else(|| ParseError::new(&String::from("Expected Operator"), &self.empty_token))
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
                value: Some(token::Literal::Bool(false)),
            });
        } else if self.match_symbol(&[TokenType::True]) {
            return Ok(Expr::Literal {
                value: Some(token::Literal::Bool(true)),
            });
        } else if self.match_symbol(&[TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: Some(token::Literal::Nil),
            });
        } else if self.match_symbol(&[TokenType::Number, TokenType::String]) {
            let prev_literal;

            match self.previous() {
                Some(token) => {
                    prev_literal = token.literal.clone();
                }
                None => prev_literal = None,
            }

            return Ok(Expr::Literal {
                value: prev_literal,
            });
        } else if self.match_symbol(&[TokenType::Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous().unwrap().clone(),
            });
        } else if self.match_symbol(&[TokenType::LeftParen]) {
            let expr = self.expression()?;

            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;

            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        } else {
            Err(ParseError::new(
                &"Expected expression.".to_string(),
                self.peek().unwrap(),
            ))
        }
    }

    /// # consume
    ///
    /// checks to see if the next token is of the expected type. If so, it consumes the token. Else, it returns a parse error.
    /// Consume means to take in the next token and advance the cursor. It returns a reference to the token that was consumed so it can be used by the interpreter.
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        match self.check(&token_type) {
            true => {
                return Ok(self.advance().unwrap());
            }
            false => match self.peek() {
                Some(next_token) => Err(self.error(next_token, message)),
                None => {
                    return Err(ParseError::new(
                        &"No token found".to_string(),
                        &self.empty_token,
                    ));
                }
            },
        }
    }

    /// # error
    ///
    /// creates a new parse error
    ///
    /// The error() method returns the error instead of throwing it because we want to let the calling method inside the parser decide whether to unwind or not.
    ///
    fn error(&self, token: &Token, message: &str) -> ParseError {
        match token.token_type {
            TokenType::Eof => {
                return ParseError::new(&"Unexpected end of file.".to_string(), token);
            }
            _ => {
                return ParseError::new(message, token);
            }
        }
    }

    /// # synchronize
    ///
    /// Catches exceptions at statement boundaries, and brings the parser to the correct state. This prevents unwanted error messages from polluting the user's dev experience.
    ///
    fn synchronize(&mut self) {
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
