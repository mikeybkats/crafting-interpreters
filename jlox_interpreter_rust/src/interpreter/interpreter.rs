use crate::scanner::{
    expr::{Expr, ExprVisitor},
    token::{Literal, Token, TokenType},
};

use super::runtime_error::RuntimeError;

pub struct Interpreter;
impl Interpreter {
    pub fn interpret(&self, expression: &Expr) -> Result<Literal, RuntimeError> {
        match self.evaluate(expression) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    fn evaluate(&self, expression: &Expr) -> Result<Literal, RuntimeError> {
        match expression.accept(&Self) {
            Ok(result) => Ok(result),
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
            (TokenType::BangEqual, left_num, right_num) => {
                Ok(Literal::Bool(!self.is_equal(&left_num, &right_num)))
            }
            (TokenType::EqualEqual, left_num, right_num) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Literal::Bool(self.is_equal(&left_num, &right_num))),
                    Err(e) => Err(e),
                }
            }

            (TokenType::Greater, Literal::Num(left_num), Literal::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Literal::Bool(left_num > right_num)),
                    Err(e) => Err(e),
                }
            }
            (TokenType::GreaterEqual, Literal::Num(left_num), Literal::Num(right_num)) => {
                Ok(Literal::Bool(left_num >= right_num))
            }
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
            (TokenType::Minus, Literal::Num(left_num), Literal::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_result) => Ok(Literal::Num(left_num - right_num)),
                    Err(e) => Err(e),
                }
            }
            (TokenType::Plus, Literal::Num(left_num), Literal::Num(right_num)) => {
                Ok(Literal::Num(left_num + right_num))
            }
            (TokenType::Plus, Literal::Str(left_str), Literal::Str(right_str)) => {
                Ok(Literal::Str(format!("{}{}", left_str, right_str)))
            }
            (TokenType::Slash, Literal::Num(left_num), Literal::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Literal::Num(left_num / right_num)),
                    Err(e) => Err(e),
                }
            }
            (TokenType::Star, Literal::Num(left_num), Literal::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Literal::Num(left_num * right_num)),
                    Err(e) => Err(e),
                }
            }
            _ => Ok(Literal::Nil),
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
}
