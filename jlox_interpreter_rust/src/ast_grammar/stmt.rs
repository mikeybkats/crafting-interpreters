use super::expr::Expr;

/// # Stmt
/// Statements form a second hierarchy of syntax tree nodes independent of expressions. We add the first couple of them in “Statements and State”.
pub enum Stmt {
    Expression { expression: Box<Expr> },
    Print { expression: Box<Expr> },
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &impl StmtVisitor<R>) -> R {
        match self {
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
        }
    }
}

pub trait StmtVisitor<R> {
    fn visit_expression_stmt(&self, expression: &Expr) -> R;
    fn visit_print_stmt(&self, expression: &Expr) -> R;
}
