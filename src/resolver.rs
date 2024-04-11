use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::{runtime_error::RuntimeError, LoxError},
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
pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// # Resolve
    /// The resolver walks the syntax tree and resolves each variable similar to the interpreter with differences:
    /// - no control flow - branching like if statements and loops have no effect
    /// - no side effects - when a function is visited the function is not actually run
    pub fn resolve(&mut self, statements: &mut Vec<Stmt>) -> Result<Object, LoxError> {
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
        if self.scopes.is_empty() {
            // variable is global
            return;
        }

        let scope = self.scopes.last_mut();
        if let Some(scope) = scope {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    /// # Resolve Local
    ///
    /// ### Stores the variable and its depth in a side table.  
    ///
    /// Each time a variable is visited (anytime a variable is accessed):
    ///
    /// The resolve_local method saves the depth of the scope between where the variable is defined and the current scope.
    ///
    /// "We start at the innermost scope and work outwards, looking in each map for a matching name. If we find the variable, we resolve it, passing in the number of scopes between the current innermost scope and the scope where the variable was found."
    fn resolve_local(&mut self, value: &Expr, name: &Token) -> Result<Object, LoxError> {
        let scopes = &self.scopes;

        for (i, scope) in scopes.into_iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                return self
                    .interpreter
                    .clone()
                    .borrow_mut()
                    .resolve(value, scopes.len() - 1 - i);
            }
        }

        Ok(Object::Nil)
    }

    /// # Resolve Function
    ///
    /// When a function is declared, the resolver creates a new scope for the function and resolves the function's body.
    fn resolve_function(&mut self, fun_stmt: &mut FunStmt) -> Result<Object, LoxError> {
        self.begin_scope();
        for param in &fun_stmt.params {
            self.declare(param);
            self.define(param);
        }

        self.resolve(&mut fun_stmt.body.statements)?;
        self.end_scope();

        Ok(Object::Nil)
    }
}

impl ExprVisitor<Result<Object, LoxError>> for Resolver {
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        _operator: &Token,
        right: &Expr,
    ) -> Result<Object, LoxError> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        _paren: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<Object, LoxError> {
        self.resolve_expr(callee)?;

        for argument in arguments {
            self.resolve_expr(argument)?;
        }

        Ok(Object::Nil)
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Object, LoxError> {
        self.resolve_expr(expression)
    }

    fn visit_literal_expr(&mut self, _value: &Option<Object>) -> Result<Object, LoxError> {
        Ok(Object::Nil)
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        _operator: &Token,
        right: &Expr,
    ) -> Result<Object, LoxError> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)
    }

    fn visit_unary_expr(&mut self, _operator: &Token, right: &Expr) -> Result<Object, LoxError> {
        self.resolve_expr(right)
    }

    fn visit_variable_expr(&mut self, variable: &Expr, name: &Token) -> Result<Object, LoxError> {
        if let Some(scope) = self.scopes.last() {
            if let Some(&false) = scope.get(&name.lexeme) {
                return Err(LoxError::RuntimeError(RuntimeError::new(
                    format!("Cannot read local variable in its own initializer."),
                    &name.clone(),
                )));
            }
        }
        self.resolve_local(variable, &name)?;
        Ok(Object::Nil)
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Object, LoxError> {
        self.resolve_expr(value)?;
        self.resolve_local(value, name)
    }
}

impl StmtVisitor<Result<Object, LoxError>> for Resolver {
    fn visit_expression_stmt(&mut self, stmt: &Expr) -> Result<Object, LoxError> {
        // stmt.accept(self)
        self.resolve_expr(stmt)
    }

    fn visit_function_stmt(&mut self, fun_stmt: &mut FunStmt) -> Result<Object, LoxError> {
        self.declare(&fun_stmt.name);
        self.define(&fun_stmt.name);
        self.resolve_function(fun_stmt)?;

        Ok(Object::Nil)
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &mut Stmt,
        else_branch: &mut Option<Box<Stmt>>,
    ) -> Result<Object, LoxError> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(then_branch)?;

        if let Some(else_branch) = else_branch {
            self.resolve_stmt(else_branch)
        } else {
            Ok(Object::Nil)
        }
    }

    fn visit_print_stmt(&mut self, expression: &Expr) -> Result<Object, LoxError> {
        self.resolve_expr(expression)
    }

    fn visit_return_stmt(&mut self, _token: &Token, value: &Expr) -> Result<Object, LoxError> {
        self.resolve_expr(value)
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &mut Stmt) -> Result<Object, LoxError> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(body)
    }

    fn visit_block_stmt(&mut self, statements: &mut BlockStmt) -> Result<Object, LoxError> {
        self.begin_scope();
        let result = self.resolve(&mut statements.statements);
        self.end_scope();

        result
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<Object, LoxError> {
        self.declare(name);
        self.resolve_expr(initializer)?;
        self.define(name);

        Ok(Object::Nil)
    }
}
