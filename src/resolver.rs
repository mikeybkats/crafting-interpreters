use std::collections::HashMap;

use crate::{
    error::LoxError,
    grammar::{
        expr::{Expr, ExprVisitor},
        object::Object,
        stmt::{BlockStmt, FunStmt, Stmt, StmtVisitor},
        token::Token,
    },
    interpreter::Interpreter,
};

/// # Resolver
///
/// The resolver visits every node in the syntax tree and resolves the scope of each variable.
///
/// It helps optimize the code by resolving variable scopes at compile time.
struct Resolver<'a> {
    interpreter: Interpreter,
    scopes: Vec<&'a mut HashMap<String, bool>>,
}

impl Resolver<'_> {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn begin_scope(&mut self) {
        unimplemented!()
    }

    fn resolve(&mut self, statements: &mut Vec<Stmt>) -> Result<Object, LoxError> {
        for statement in statements {
            self.resolve_stmt(statement)?;
        }

        Ok(Object::Nil)
    }

    fn resolve_stmt(&mut self, statement: &mut Stmt) -> Result<Object, LoxError> {
        statement.accept(self)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    /// # Declare
    /// "As we visit expressions, we need to know if we’re inside the initializer for some variable. We do that by splitting binding into two steps. The first is declaring it."
    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.last_mut();

        if let Some(scope) = scope {
            scope.insert(name.lexeme.clone(), false);
        }
    }

    /// # Define
    /// "After declaring the variable, we resolve its initializer expression in that same scope where the new variable now exists but is unavailable."
    fn define(&mut self, name: &Token) {
        unimplemented!()
    }
}

impl ExprVisitor<Result<Object, LoxError>> for Resolver<'_> {
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

impl StmtVisitor<Result<Object, LoxError>> for Resolver<'_> {
    fn visit_expression_stmt(&mut self, stmt: &Expr) -> Result<Object, LoxError> {
        stmt.accept(self)
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
        _body: &mut Stmt,
    ) -> Result<Object, LoxError> {
        unimplemented!()
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<Object, LoxError> {
        self.declare(name);
        self.resolve_expr(initializer)
    }

    fn visit_block_stmt(&mut self, statements: &mut BlockStmt) -> Result<Object, LoxError> {
        self.begin_scope();
        let result = self.resolve(&mut statements.statements);
        self.end_scope();

        result
    }
}
