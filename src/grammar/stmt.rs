use super::{expr::Expr, token::Token};

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct FunStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ClassStmt {
    pub name: Token,
    pub methods: Vec<FunStmt>,
}

#[derive(Debug, Clone)]
/// # Stmt
/// Statements form a second hierarchy of syntax tree nodes independent of expressions. We add the first couple of them in “Statements and State”.
pub enum Stmt {
    Class(ClassStmt),
    Block(BlockStmt),
    Function(FunStmt),
    Getter(FunStmt),
    Expression {
        expression: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        expression: Box<Expr>,
    },
    Return {
        keyword: Token,
        value: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Box<Expr>,
    },
    // If you're wondering why no For statements? They are handled in the parser because they are just desugared into while loops.
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
}

impl Stmt {
    pub fn accept<R>(&mut self, visitor: &mut impl StmtVisitor<R>) -> R {
        match self {
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if_stmt(condition, then_branch, else_branch),
            Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
            Stmt::Return { keyword, value } => visitor.visit_return_stmt(keyword, value),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
            Stmt::Block(block_stmt) => visitor.visit_block_stmt(block_stmt),
            Stmt::Function(fun_stmt) => visitor.visit_function_stmt(fun_stmt),
            Stmt::Getter(getter_stmt) => visitor.visit_getter_stmt(getter_stmt),
            Stmt::Class(class_stmt) => visitor.visit_class_stmt(class_stmt),
        }
    }
}

pub trait StmtVisitor<R> {
    fn visit_expression_stmt(&mut self, expression: &Expr) -> R;
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &mut Stmt,
        else_branch: &mut Option<Box<Stmt>>,
    ) -> R;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &mut Stmt) -> R;
    fn visit_print_stmt(&mut self, expression: &Expr) -> R;
    fn visit_return_stmt(&mut self, keyword: &Token, value: &Expr) -> R;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> R;
    fn visit_block_stmt(&mut self, statements: &mut BlockStmt) -> R;
    fn visit_function_stmt(&mut self, fun_stmt: &mut FunStmt) -> R;
    fn visit_getter_stmt(&mut self, getter_stmt: &mut FunStmt) -> R;
    fn visit_class_stmt(&mut self, class_stmt: &ClassStmt) -> R;
}
