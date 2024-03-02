use super::object::Object;
use super::token::Token;

#[derive(Debug, Clone)]
/// # Expression
/// Enumerates the different types of expressions.
///
/// ## Differences between Rust and Java implementations
/// in the book, the author uses a GenerateAST.java class to generate the AST classes. This is not necessary in Rust. The enum and struct syntax achieves the same result in a more straightforward way.
///
/// ## First appears in Representing Code (Chapter 5)
/// "Expressions are the first syntax tree nodes we see, introduced in the _Representing Code_ chapter. The main Expr class defines the visitor interface used to dispatch against the specific expression types, and contains the other expression subclasses as nested classes."
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Option<Object>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut impl ExprVisitor<R>) -> R {
        match self {
            Expr::Assign { name, value } => visitor.visit_assign_expr(name, value),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call_expr(callee, paren, arguments),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_object_expr(value),
            Expr::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical_expr(left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            Expr::Variable { name } => visitor.visit_variable_expr(name),
        }
    }
}

pub trait ExprVisitor<R> {
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> R;
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Expr>) -> R;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> R;
    fn visit_object_expr(&mut self, value: &Option<Object>) -> R;
    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> R;
    fn visit_variable_expr(&mut self, name: &Token) -> R;
}
