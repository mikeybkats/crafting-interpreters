use crate::scanner::{
    expr::{Expr, ExprVisitor},
    token::{Literal, Token, TokenType},
};

///
/// # LoxObject
/// "The leaves of an expression tree—the atomic bits of syntax that all other expressions are composed of—are literals. Literals are almost values already, but the distinction is important. A literal is a bit of syntax that produces a value. A literal always appears somewhere in the user’s source code. Lots of values are produced by computation and don’t exist anywhere in the code itself. Those aren’t literals. A literal comes from the parser’s domain. Values are an interpreter concept, part of the runtime’s world." [CraftingInterpreters](https://craftinginterpreters.com/evaluating-expressions.html)
// #[derive(Debug, Clone)]
// pub struct LoxObject {
//     value: Literal,
// }

pub struct Interpreter;
impl Interpreter {
    fn evaluate(&self, expression: &Expr) -> Literal {
        expression.accept(&Self)
    }
}

impl ExprVisitor<Literal> for Interpreter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> Literal {
        Literal::Nil
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
