use super::{expr::Expr, token::Token};

#[derive(Debug, Clone)]
/// # Stmt
/// Statements form a second hierarchy of syntax tree nodes independent of expressions. We add the first couple of them in “Statements and State”.
pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Expression {
        expression: Box<Expr>,
    },
    // For {
    //     initializer: Box<Stmt>,
    //     condition: Box<Expr>,
    //     increment: Box<Expr>,
    //     body: Box<Stmt>,
    // },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        expression: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Box<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
}

impl Stmt {
    pub fn accept<R>(&mut self, visitor: &mut impl StmtVisitor<R>) -> R {
        match self {
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            // Stmt::For { initializer, condition, increment, body } => {
            //     visitor.visit_for_stmt(initializer, condition, increment, body)
            // }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if_stmt(condition, then_branch, else_branch),
            Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
        }
    }
}

pub trait StmtVisitor<R> {
    fn visit_expression_stmt(&mut self, expression: &Expr) -> R;
    // fn visit_for_stmt(
    //     &mut self,
    //     initializer: &mut Stmt,
    //     condition: &Expr,
    //     increment: &Expr,
    //     body: &mut Stmt,
    // ) -> R;
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &mut Stmt,
        else_branch: &mut Option<Box<Stmt>>,
    ) -> R;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &mut Stmt) -> R;
    fn visit_print_stmt(&mut self, expression: &Expr) -> R;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> R;
    fn visit_block_stmt(&mut self, statements: &mut Vec<Stmt>) -> R;
}
