use crate::ast_grammar::expr::{Expr, ExprVisitor};
use crate::ast_grammar::stmt::{Stmt, StmtVisitor};
use crate::ast_grammar::token::{Literal, Token, TokenType};
use crate::error::runtime_error::RuntimeError;

pub struct Interpreter;
impl Interpreter {
    pub fn interpret(&self, statements: Vec<Stmt>) -> Result<Literal, RuntimeError> {
        for statement in statements {
            match self.execute(&statement) {
                Ok(value) => return Ok(value),
                Err(e) => return Err(e),
            }
        }

        Ok(Literal::Nil)
    }

    pub fn execute(&self, statement: &Stmt) -> Result<Literal, RuntimeError> {
        match statement.accept(&Self) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub fn evaluate(&self, expression: &Expr) -> Result<Literal, RuntimeError> {
        match expression.accept(&Self) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    fn is_equal(&self, a: &Literal, b: &Literal) -> bool {
        match (a, b) {
            (Literal::Num(a), Literal::Num(b)) => a == b,
            (Literal::Str(a), Literal::Str(b)) => a == b,
            (Literal::Bool(a), Literal::Bool(b)) => a == b,
            _ => false,
        }
    }

    fn check_number_operand(
        &self,
        token: &Token,
        operand: Literal,
    ) -> Result<Literal, RuntimeError> {
        match operand {
            Literal::Num(_) => Ok(operand),
            _ => {
                return Err(RuntimeError::new(
                    "Operand must be a number.".to_string(),
                    token,
                ))
            }
        }
    }

    fn check_number_operands(
        &self,
        token: &Token,
        left: Literal,
        right: Literal,
    ) -> Result<bool, RuntimeError> {
        match (left, right) {
            (Literal::Num(_left), Literal::Num(_right)) => Ok(true),
            _ => {
                return Err(RuntimeError::new(
                    "Operands must be numbers.".to_string(),
                    token,
                ))
            }
        }
    }
}

impl ExprVisitor<Result<Literal, RuntimeError>> for Interpreter {
    fn visit_binary_expr(
        &self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match (operator.token_type, left.clone(), right.clone()) {
            // Handle equals
            (TokenType::BangEqual, left_num, right_num) => {
                Ok(Literal::Bool(!self.is_equal(&left_num, &right_num)))
            }
            (TokenType::EqualEqual, left_num, right_num) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Literal::Bool(self.is_equal(&left_num, &right_num))),
                    Err(e) => Err(e),
                }
            }

            // Handle greater than
            (TokenType::Greater, Literal::Num(left_num), Literal::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Literal::Bool(left_num > right_num)),
                    Err(e) => {
                        // TODO: handle error
                        Err(e)
                    }
                }
            }
            (TokenType::GreaterEqual, Literal::Num(left_num), Literal::Num(right_num)) => {
                Ok(Literal::Bool(left_num >= right_num))
            }

            // Handle less than
            (TokenType::Less, Literal::Num(left_num), Literal::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Literal::Bool(left_num < right_num)),
                    Err(e) => Err(e),
                }
            }
            (TokenType::LessEqual, Literal::Num(left_num), Literal::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Literal::Bool(left_num <= right_num)),
                    Err(e) => Err(e),
                }
            }

            // Handle subtraction
            (TokenType::Minus, Literal::Num(left_num), Literal::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_result) => Ok(Literal::Num(left_num - right_num)),
                    Err(e) => Err(e),
                }
            }

            // Handle addition
            (TokenType::Plus, Literal::Num(left_num), Literal::Num(right_num)) => {
                Ok(Literal::Num(left_num + right_num))
            }
            (TokenType::Plus, Literal::Str(left_str), Literal::Str(right_str)) => {
                Ok(Literal::Str(format!("{}{}", left_str, right_str)))
            }

            // Handle addition of string and number concatenation
            (TokenType::Plus, Literal::Num(left_num), Literal::Str(right_str)) => {
                Ok(Literal::Str(format!("{}{}", left_num, right_str)))
            }
            (TokenType::Plus, Literal::Str(left_str), Literal::Num(right_num)) => {
                Ok(Literal::Str(format!("{}{}", left_str, right_num)))
            }

            // Handle division
            (TokenType::Slash, Literal::Num(left_num), Literal::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Literal::Num(left_num / right_num)),
                    Err(e) => Err(e),
                }
            }

            // Handle multiplication
            (TokenType::Star, Literal::Num(left_num), Literal::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Literal::Num(left_num * right_num)),
                    Err(e) => Err(e),
                }
            }

            // Handle errors
            _ => Err(RuntimeError::new(
                format!(
                    "Expression: '{} {} {}' does not evaluate.",
                    left.format(),
                    operator.lexeme,
                    right.format(),
                ),
                operator,
            )),
        }
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> Result<Literal, RuntimeError> {
        self.evaluate(expression)
    }

    fn visit_literal_expr(&self, value: &Option<Literal>) -> Result<Literal, RuntimeError> {
        let empty_token = Token::new(TokenType::Nil, "".to_string(), Some(Literal::Nil), 0);

        match value {
            Some(value) => Ok(value.clone()),
            _ => Err(RuntimeError::new("No value".to_string(), &empty_token)),
        }
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Result<Literal, RuntimeError> {
        let right_literal = self.evaluate(right)?;
        let empty_token = Token::new(TokenType::Nil, "".to_string(), Some(Literal::Nil), 0);

        match (operator.token_type, right_literal.clone()) {
            (TokenType::Minus, Literal::Num(num)) => {
                match self.check_number_operand(operator, right_literal) {
                    Ok(_result) => Ok(Literal::Num(-num)),
                    Err(e) => Err(e),
                }
            }
            (TokenType::Bang, _) => Ok(Literal::Bool(!right_literal.is_truthy())),
            _ => Err(RuntimeError::new("No value".to_string(), &empty_token)),
        }
    }

    fn visit_variable_expr(&self, _name: &Token) -> Result<Literal, RuntimeError> {
        // TODO: UPDATE THIS
        let empty_token = Token::new(TokenType::Nil, "".to_string(), Some(Literal::Nil), 0);
        Err(RuntimeError::new("No value".to_string(), &empty_token))
    }
}

impl StmtVisitor<Result<Literal, RuntimeError>> for Interpreter {
    fn visit_expression_stmt(&self, statement: &Expr) -> Result<Literal, RuntimeError> {
        self.evaluate(statement)
    }

    fn visit_print_stmt(&self, statement: &Expr) -> Result<Literal, RuntimeError> {
        let value = self.evaluate(statement)?;
        println!("{}", value.format());
        Ok(Literal::Nil)
    }

    fn visit_var_stmt(&self, _name: &Token, _initializer: &Expr) -> Result<Literal, RuntimeError> {
        // TODO: UPDATE THIS
        // let value = self.evaluate(initializer)?;
        // println!("{}: {}", name.lexeme, value.format());
        Ok(Literal::Nil)
    }
}