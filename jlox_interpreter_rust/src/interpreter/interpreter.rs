use crate::scanner::{
    expr::{Expr, ExprVisitor},
    token::{Literal, Token, TokenType},
};

use super::runtime_error::RuntimeError;

pub struct Interpreter;
impl Interpreter {
    fn evaluate(&self, expression: &Expr) -> Literal {
        expression.accept(&Self)
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
        token_type: TokenType,
        operand: Literal,
    ) -> Result<Literal, RuntimeError> {
        match operand {
            Literal::Num(_) => Ok(operand),
            _ => {
                return Err(RuntimeError::new(
                    "Operand must be a number.".to_string(),
                    operand,
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
        let left = self.evaluate(left);
        let right = self.evaluate(right);

        match (operator.token_type, left, right) {
            // (TokenType::BangEqual, left_num, right_num) => {
            //     Literal::Bool(!self.is_equal(&left_num, &right_num))
            // }
            // (TokenType::EqualEqual, left_num, right_num) => {
            //     Literal::Bool(self.is_equal(&left_num, &right_num))
            // }

            // (TokenType::Greater, Literal::Num(left_num), Literal::Num(right_num)) => {
            //     Literal::Bool(left_num > right_num)
            // }
            // (TokenType::GreaterEqual, Literal::Num(left_num), Literal::Num(right_num)) => {
            //     Literal::Bool(left_num >= right_num)
            // }
            // (TokenType::Less, Literal::Num(left_num), Literal::Num(right_num)) => {
            //     Literal::Bool(left_num < right_num)
            // }
            // (TokenType::LessEqual, Literal::Num(left_num), Literal::Num(right_num)) => {
            //     Literal::Bool(left_num <= right_num)
            // }
            (TokenType::Minus, Literal::Num(left_num), Literal::Num(right_num)) => {
                match self.check_number_operand(TokenType::Minus, right) {
                    Ok(result) => Ok(Literal::Num(left_num - right_num)),
                    Err(e) => Err(e),
                }
            }
            // (TokenType::Plus, Literal::Num(left_num), Literal::Num(right_num)) => {
            //     Literal::Num(left_num + right_num)
            // }
            // (TokenType::Plus, Literal::Str(left_str), Literal::Str(right_str)) => {
            //     Literal::Str(format!("{}{}", left_str, right_str))
            // }
            // (TokenType::Slash, Literal::Num(left_num), Literal::Num(right_num)) => {
            //     Literal::Num(left_num / right_num)
            // }
            // (TokenType::Star, Literal::Num(left_num), Literal::Num(right_num)) => {
            //     Literal::Num(left_num * right_num)
            // }
            _ => Ok(Literal::Nil),
        }
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> Literal {
        self.evaluate(expression)
    }

    fn visit_literal_expr(&self, value: &Option<Literal>) -> Literal {
        match value {
            Some(value) => value.clone(),
            _ => Literal::Nil,
        }
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Literal {
        let right_literal = self.evaluate(right);

        match (operator.token_type, right_literal.clone()) {
            (TokenType::Minus, Literal::Num(num)) => Literal::Num(-num),
            (TokenType::Bang, _) => Literal::Bool(!right_literal.is_truthy()),
            _ => Literal::Nil,
        }
    }
}
