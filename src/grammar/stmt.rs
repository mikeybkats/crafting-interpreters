use super::{
    expr::Expr,
    token::{Token, TokenType},
};

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
/// # Stmt
/// Statements form a second hierarchy of syntax tree nodes independent of expressions. We add the first couple of them in “Statements and State”.
pub enum Stmt {
    Block(BlockStmt),
    Expression {
        expression: Box<Expr>,
    },
    // If you're wondering why no For statements? They are handled in the parser because they are just desugared into while loops.
    Function {
        name: Token,
        params: Vec<TokenType>,
        body: BlockStmt,
    },
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
            Stmt::Function { name, params, body } => {
                visitor.visit_function_stmt(name, params, body)
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if_stmt(condition, then_branch, else_branch),
            Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
            Stmt::Block(block_stmt) => visitor.visit_block_stmt(block_stmt),
        }
    }
}

pub trait StmtVisitor<R> {
    fn visit_expression_stmt(&mut self, expression: &Expr) -> R;
    fn visit_function_stmt(
        &mut self,
        name: &Token,
        params: &mut Vec<TokenType>,
        body: &BlockStmt,
    ) -> R;
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &mut Stmt,
        else_branch: &mut Option<Box<Stmt>>,
    ) -> R;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &mut Stmt) -> R;
    fn visit_print_stmt(&mut self, expression: &Expr) -> R;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> R;
    fn visit_block_stmt(&mut self, statements: &mut BlockStmt) -> R;
}
