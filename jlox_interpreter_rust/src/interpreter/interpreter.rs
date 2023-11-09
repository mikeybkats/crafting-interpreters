use crate::scanner::{expr::ExprVisitor, token::Literal};

///
/// # LoxObject
/// "The leaves of an expression tree—the atomic bits of syntax that all other expressions are composed of—are literals. Literals are almost values already, but the distinction is important. A literal is a bit of syntax that produces a value. A literal always appears somewhere in the user’s source code. Lots of values are produced by computation and don’t exist anywhere in the code itself. Those aren’t literals. A literal comes from the parser’s domain. Values are an interpreter concept, part of the runtime’s world." [CraftingInterpreters](https://craftinginterpreters.com/evaluating-expressions.html)
#[derive(Debug, Clone)]
pub struct LoxObject {
    value: Literal,
}

pub struct Interpreter {
    value: LoxObject,
}
impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            value: LoxObject {
                value: Literal::Nil,
            },
        }
    }
}

impl ExprVisitor<LoxObject> for Interpreter {
    fn visit_binary_expr(
        &self,
        left: &crate::scanner::expr::Expr,
        operator: &crate::scanner::token::Token,
        right: &crate::scanner::expr::Expr,
    ) -> LoxObject {
        self.value.clone()
    }
    fn visit_grouping_expr(&self, expression: &crate::scanner::expr::Expr) -> LoxObject {
        self.value.clone()
    }
    fn visit_literal_expr(&self, value: &Option<Literal>) -> LoxObject {
        self.value.clone()
    }
    fn visit_unary_expr(
        &self,
        operator: &crate::scanner::token::Token,
        right: &crate::scanner::expr::Expr,
    ) -> LoxObject {
        self.value.clone()
    }
}
