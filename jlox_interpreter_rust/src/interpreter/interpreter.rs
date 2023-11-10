use crate::scanner::{
    expr::{Expr, ExprVisitor},
    token::{Literal, Token},
};

///
/// # LoxObject
/// "The leaves of an expression tree—the atomic bits of syntax that all other expressions are composed of—are literals. Literals are almost values already, but the distinction is important. A literal is a bit of syntax that produces a value. A literal always appears somewhere in the user’s source code. Lots of values are produced by computation and don’t exist anywhere in the code itself. Those aren’t literals. A literal comes from the parser’s domain. Values are an interpreter concept, part of the runtime’s world." [CraftingInterpreters](https://craftinginterpreters.com/evaluating-expressions.html)
#[derive(Debug, Clone)]
pub struct LoxObject {
    value: Literal,
}

pub struct Interpreter;
impl Interpreter {
    // fn new() -> Interpreter {
    //     Interpreter {
    //         value: LoxObject {
    //             value: Literal::Nil,
    //         },
    //     }
    // }

    fn evaluate(&self, expression: &Expr) -> LoxObject {
        expression.accept(&Self)
    }
}

impl ExprVisitor<LoxObject> for Interpreter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> LoxObject {
        // match left {
        //     Expr::Binary { left, operator, right } => {
        //         LoxObject {
        //             value: left,
        //         }
        //     }
        // }
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> LoxObject {
        self.evaluate(expression)
    }
    fn visit_literal_expr(&self, value: &Option<Literal>) -> LoxObject {
        match value {
            Some(value) => LoxObject {
                value: value.clone(),
            },
            _ => LoxObject {
                value: Literal::Nil,
            },
        }
    }
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> LoxObject {}
}
