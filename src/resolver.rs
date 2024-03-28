use crate::{
    error::LoxError,
    grammar::{
        expr::{Expr, ExprVisitor},
        object::Object,
        stmt::{FunStmt, StmtVisitor},
        token::Token,
    },
    interpreter::Interpreter,
};

/// # Resolver
///
/// The resolver visits every node in the syntax tree and resolves the scope of each variable.
///
/// It helps optimize the code by resolving variable scopes at compile time.
struct Resolver {
    interpreter: Interpreter,
}

impl ExprVisitor<Result<Object, LoxError>> for Resolver {
    fn visit_binary_expr(
        &mut self,
        _left: &Expr,
        _operator: &Token,
        _right: &Expr,
    ) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_call_expr(
        &mut self,
        _callee: &Expr,
        _paren: &Token,
        _arguments: &Vec<Expr>,
    ) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_grouping_expr(&mut self, _expression: &Expr) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_object_expr(&mut self, _value: &Option<Object>) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_logical_expr(
        &mut self,
        _left: &Expr,
        _operator: &Token,
        _right: &Expr,
    ) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_unary_expr(&mut self, _operator: &Token, _right: &Expr) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_variable_expr(&mut self, _name: &Token) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_assign_expr(&mut self, _name: &Token, _value: &Expr) -> Result<Object, LoxError> {
        unimplemented!()
    }
}

impl StmtVisitor<Result<Object, LoxError>> for Resolver {
    fn visit_expression_stmt(&mut self, _stmt: &Expr) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_function_stmt(&mut self, _fun_stmt: &mut FunStmt) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_if_stmt(
        &mut self,
        _condition: &Expr,
        _then_branch: &mut crate::grammar::stmt::Stmt,
        _else_branch: &mut Option<Box<crate::grammar::stmt::Stmt>>,
    ) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_print_stmt(&mut self, _expression: &Expr) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_return_stmt(&mut self, _value: &Expr) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_while_stmt(
        &mut self,
        _condition: &Expr,
        _body: &mut crate::grammar::stmt::Stmt,
    ) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_var_stmt(
        &mut self,
        _name: &crate::grammar::token::Token,
        _initializer: &Expr,
    ) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_block_stmt(
        &mut self,
        _statements: &mut crate::grammar::stmt::BlockStmt,
    ) -> Result<Object, LoxError> {
        unimplemented!()
    }
}
