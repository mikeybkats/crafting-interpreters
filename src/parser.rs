use std::vec;

use crate::environment::generate_id;
use crate::error::parse_error::ParseError;
use crate::grammar::object::Object;
use crate::grammar::stmt::{BlockStmt, ClassStmt, FunStmt, FunType, Stmt};
use crate::grammar::token::{Token, TokenType};

use crate::grammar::expr::{Expr, Variable};

/// # Parser
///
/// Takes a list of tokens and parses them into an abstract syntax tree (AST), which the interpreter uses to evaluate the program.
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
            empty_token: Token::new(
                TokenType::Nil,
                "".to_string(),
                Some(Object::Nil),
                0,
                generate_id(),
            ),
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
        self.assignment()
    }

    /// # declaration
    /// Called repeatedly when parsing statments in either block or script mode
    /// This is the where the application should synchronize when the parser panics.
    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_symbol(&[TokenType::Class]) {
            return self.class_declaration();
        }
        if self.match_symbol(&[TokenType::Fun]) {
            return self.function("function");
        }
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

    fn class_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name_check = self.consume(TokenType::Identifier, "Expect class name.")?;
        let name = name_check.clone();

        let mut superclass: Option<Variable> = None;

        if self.match_symbol(&[TokenType::Less]) {
            self.consume(TokenType::Identifier, "Expect superclass name.")?;
            superclass = Some(Variable {
                name: self.previous().unwrap().clone(),
            });
        }

        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;

        let mut methods: Vec<FunStmt> = vec![];

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            match self.function("method") {
                Ok(method) => match method {
                    Stmt::Function(fun_stmt) => {
                        methods.push(fun_stmt);
                    }
                    _ => {
                        return Err(ParseError::new(
                            &"Expected method declaration.".to_string(),
                            self.peek().unwrap(),
                        ));
                    }
                },
                Err(e) => return Err(e),
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;

        return Ok(Stmt::Class(ClassStmt {
            name,
            superclass,
            methods,
        }));
    }

    /// # statement
    /// "A program is a list of statements, and we parse one of those statements using this method"
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_symbol(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.match_symbol(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.match_symbol(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_symbol(&[TokenType::Return]) {
            return self.return_statement();
        }
        if self.match_symbol(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.match_symbol(&[TokenType::LeftBrace]) {
            return match self.block() {
                Ok(block) => Ok(Stmt::Block(block)),
                Err(e) => Err(e),
            };
        }

        self.expression_statement()
    }

    /// # for_statement
    /// parse a for statement
    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.match_symbol(&[TokenType::Semicolon]) {
            None
        } else if self.match_symbol(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let mut condition: Option<Expr> = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment: Option<Expr> = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement();

        if let Some(increment) = increment {
            let mut body_statements = vec![];
            if let Ok(body) = body {
                body_statements = match body {
                    Stmt::Block(block) => block.statements,
                    _ => vec![body],
                };
            }
            body = Ok(Stmt::Block(BlockStmt {
                statements: {
                    let mut v = body_statements;
                    v.push(Stmt::Expression {
                        expression: Box::new(increment),
                    });
                    v
                },
            }))
        }

        if let None = condition {
            condition = Some(Expr::Literal {
                value: Some(Object::Bool(true)),
            })
        }

        body = Ok(Stmt::While {
            condition: Box::new(condition.unwrap()),
            body: Box::new(body.unwrap()),
        });

        if let Some(initializer) = initializer {
            body = Ok(Stmt::Block(BlockStmt {
                statements: vec![initializer, body.unwrap()],
            }))
        }

        return body;
    }

    /// # if_statement
    ///
    /// parse an if statement
    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;

        let condition = self.expression()?;

        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_symbol(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        })
    }

    /// # while_statement
    ///
    /// parse a while statementd
    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;

        let condition = self.expression()?;

        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;

        let body = Box::new(self.statement()?);

        Ok(Stmt::While {
            condition: Box::new(condition),
            body,
        })
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

    /// # return_statement
    /// parse a return statement
    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous().unwrap().clone();
        let value = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;

        Ok(Stmt::Return {
            keyword,
            value: Box::new(value.unwrap_or_else(|| Expr::Literal {
                value: Some(Object::Nil),
            })),
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
            initializer: Box::new(initializer.unwrap_or_else(|| Expr::Literal {
                value: Some(Object::Nil),
            })),
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

    fn getter(&mut self, name: Token, kind: &str) -> Result<Stmt, ParseError> {
        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;

        Ok(Stmt::Function(FunStmt {
            name,
            params: vec![],
            kind: FunType::Getter,
            body: self
                .block()
                .unwrap_or_else(|_error| BlockStmt { statements: vec![] })
                .statements,
        }))
    }

    /// # function
    /// parse a function declaration
    fn function(&mut self, kind: &str) -> Result<Stmt, ParseError> {
        let name = self
            .consume(TokenType::Identifier, &format!("Expect {} name.", kind))?
            .clone();

        // if there is not left paren, it must be a getter
        if let Some(token) = self.peek() {
            if let TokenType::LeftBrace = token.token_type {
                return self.getter(name, kind);
            }
        }

        self.consume(
            TokenType::LeftParen,
            &format!("Parser::Function() -- Expect '(' after {} name.", kind),
        )?;

        let mut parameters: Vec<Token> = vec![];

        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(ParseError::new(
                        &"Can't have more than 255 parameters.".to_string(),
                        self.peek().unwrap(),
                    ));
                }

                let param = self.consume(TokenType::Identifier, "Expect parameter name.")?;

                parameters.push(param.clone());

                if !self.match_symbol(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RightParen,
            &format!("Expect ')' after {} parameters.", kind),
        )?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;

        Ok(Stmt::Function(FunStmt {
            name,
            params: parameters,
            kind: FunType::Function,
            body: self
                .block()
                .unwrap_or_else(|_error| BlockStmt { statements: vec![] })
                .statements,
        }))
    }

    fn block(&mut self) -> Result<BlockStmt, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(BlockStmt { statements })
    }

    /// # assignment
    /// parse an assignment. Will fail if the variable does not exist.
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.match_symbol(&[TokenType::Equal]) {
            let equals = self.previous().unwrap().clone();
            let value = self.assignment()?;

            match expr {
                Expr::Variable(variable) => match variable {
                    Variable { name } => {
                        return Ok(Expr::Assign {
                            name,
                            value: Box::new(value),
                        });
                    }
                },
                Expr::Get { object, name } => {
                    return Ok(Expr::Set {
                        object,
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

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.match_symbol(&[TokenType::Or]) {
            let operator = self.previous().unwrap().clone();
            let right = self.and()?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.match_symbol(&[TokenType::And]) {
            let operator = self.previous().unwrap().clone();
            let right = self.equality()?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
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
    /// Loops through a given vector of tokens, if any of the tokens in the vector match the next token the cursor advances and the method returns true .
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

        self.call()
    }

    /// # call
    /// Parses a call expression
    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_symbol(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_symbol(&[TokenType::Dot]) {
                let name =
                    self.consume(TokenType::Identifier, "Expect property name after '.'.")?;
                expr = Expr::Get {
                    object: Box::new(expr),
                    name: name.clone(),
                };
            } else {
                break;
            }
        }

        return Ok(expr);
    }

    /// # finish_call
    ///
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments: Vec<Expr> = vec![];

        if !self.check(&TokenType::RightParen) {
            loop {
                // TODO: finish call checks for expressions. This needs to change so that annonymous functions can be passed as arguments.
                // So that a function declaration could be an argument to another function.
                // This is a feature that is not yet implemented.
                arguments.push(self.expression()?);

                if arguments.len() >= 255 {
                    return Err(ParseError::new(
                        &"Can't have more than 255 arguments.".to_string(),
                        self.peek().unwrap(),
                    ));
                }
                if !self.match_symbol(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren: self
                .consume(TokenType::RightParen, "Expect ')' after arguments.")?
                .clone(),
            arguments,
        })
    }

    /// # Primary
    ///
    /// Primary is the highest level of precedence. This rule processes Objectstrings and numbers as well as booleans and all expressions:
    ///
    /// _rule_:
    ///
    /// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    ///
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_symbol(&[TokenType::False]) {
            return Ok(Expr::Literal {
                value: Some(Object::Bool(false)),
            });
        } else if self.match_symbol(&[TokenType::True]) {
            return Ok(Expr::Literal {
                value: Some(Object::Bool(true)),
            });
        } else if self.match_symbol(&[TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: Some(Object::Nil),
            });
        } else if self.match_symbol(&[TokenType::Number, TokenType::String]) {
            let prev_object;

            match self.previous() {
                Some(token) => {
                    prev_object = token.literal.clone();
                }
                None => prev_object = None,
            }

            return Ok(Expr::Literal { value: prev_object });
        } else if self.match_symbol(&[TokenType::Super]) {
            let keyword = self.previous().unwrap().clone();
            self.consume(TokenType::Dot, "Expect '.' after 'super'.")?;
            let method = self.consume(TokenType::Identifier, "Expect a superclass method name")?;
            return Ok(Expr::Super {
                keyword,
                method: method.clone(),
            });
        } else if self.match_symbol(&[TokenType::This]) {
            return Ok(Expr::This {
                keyword: self.previous().unwrap().clone(),
            });
        } else if self.match_symbol(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(Variable {
                name: self.previous().unwrap().clone(),
            }));
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

#[cfg(test)]
mod tests {
    use colored::*;

    use super::*;
    use crate::grammar::object::Object;
    use crate::grammar::token::Token;
    use crate::grammar::token::TokenType;

    #[test]
    fn test_finish_call() {
        println!(
            "{} {}",
            "test_call:".green(),
            "The finish_call function should parse arguments return a Call expression from tokens 'foo()'".red()
        );
        let tokens = vec![
            Token::new(
                TokenType::Identifier,
                "foo".to_string(),
                None,
                0,
                generate_id(),
            ),
            Token::new(
                TokenType::LeftParen,
                "(".to_string(),
                None,
                0,
                generate_id(),
            ),
            Token::new(
                TokenType::Identifier,
                "arg1".to_string(),
                None,
                0,
                generate_id(),
            ),
            Token::new(TokenType::Comma, ",".to_string(), None, 0, generate_id()),
            Token::new(
                TokenType::Identifier,
                "arg2".to_string(),
                None,
                0,
                generate_id(),
            ),
            Token::new(
                TokenType::RightParen,
                ")".to_string(),
                None,
                0,
                generate_id(),
            ),
            Token::new(
                TokenType::Semicolon,
                ";".to_string(),
                None,
                0,
                generate_id(),
            ),
        ];
        let mut parser = Parser::new(&tokens);
        let callee = parser.primary().unwrap();

        println!("{} {:#?}", "callee:".blue(), callee);
        parser.current = tokens.len() - 2;

        let result = parser.finish_call(callee).unwrap_or_else(|result| {
            println!("Error: {:#?}", result);
            return Expr::Literal {
                value: Some(Object::Nil),
            };
        });

        println!("{} {:#?}", "result:".blue(), result);

        match result {
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                assert_eq!(paren.token_type, TokenType::RightParen);
                assert_eq!(arguments.len(), 0);
                match *callee {
                    Expr::Variable(variable) => {
                        assert_eq!(variable.name.lexeme, "foo");
                    }
                    _ => {
                        panic!("Expected variable expression");
                    }
                }
            }
            _ => {
                panic!("Expected call expression");
            }
        }
    }

    #[test]
    fn test_call() {
        println!(
            "{} {}",
            "test_call:".green(),
            "The call function should return a Call expression from tokens 'foo()'".blue()
        );
        let tokens = vec![
            Token::new(
                TokenType::Identifier,
                "foo".to_string(),
                None,
                0,
                generate_id(),
            ),
            Token::new(
                TokenType::LeftParen,
                "(".to_string(),
                None,
                0,
                generate_id(),
            ),
            Token::new(
                TokenType::RightParen,
                ")".to_string(),
                None,
                0,
                generate_id(),
            ),
            Token::new(
                TokenType::Semicolon,
                ";".to_string(),
                None,
                0,
                generate_id(),
            ),
        ];
        let mut parser = Parser::new(&tokens);
        let result = parser.call().unwrap_or_else(|result| {
            println!("Error: {:#?}", result);
            return Expr::Literal {
                value: Some(Object::Nil),
            };
        });

        println!("{} {:#?}", "result:".green(), result);

        match result {
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                assert_eq!(paren.token_type, TokenType::RightParen);
                assert_eq!(arguments.len(), 0);
                match *callee {
                    Expr::Variable(variable) => {
                        assert_eq!(variable.name.lexeme, "foo");
                    }
                    _ => {
                        panic!("Expected variable expression");
                    }
                }
            }
            _ => {
                panic!("Expected call expression");
            }
        }
    }
}
