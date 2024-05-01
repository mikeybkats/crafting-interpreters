use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::{runtime_error::RuntimeError, LoxError},
    grammar::{
        expr::{Expr, ExprVisitor},
        object::Object,
        stmt::{BlockStmt, ClassStmt, FunStmt, Stmt, StmtVisitor},
        token::Token,
    },
    interpreter::Interpreter,
};

#[derive(Debug, Clone)]
enum ClassType {
    None,
    Class,
}

#[derive(Debug, Clone)]
pub enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

// TODO: Improvement -- Add scope type to the HashMap. Save index of the scope for use in the interpreter.
// struct Scope {
//     defined: bool,
//     index: usize,
// }

/// # Resolver
///
/// The resolver visits every node in the syntax tree and resolves the scope of each variable.
///
/// It helps optimize the code by resolving variable scopes at compile time.
pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    // current_class "value tells us if we are currently inside a class declaration while traversing the syntax tree"
    current_class: ClassType,
}

impl Resolver {
    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
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
    /// ### Stores the variable and its depth in a "scopes" side table.  
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
                return self.interpreter.clone().borrow_mut().resolve(value, i);
            }
        }

        Ok(Object::Nil)
    }

    /// # Resolve Function
    ///
    /// When a function is declared, the resolver creates a new scope for the function and resolves the function's body.
    fn resolve_function(
        &mut self,
        fun_stmt: &mut FunStmt,
        fun_type: FunctionType,
    ) -> Result<Object, LoxError> {
        let enclosing_function: FunctionType = self.current_function.clone();
        self.current_function = fun_type;

        self.begin_scope();
        for param in &fun_stmt.params {
            self.declare(param);
            self.define(param);
        }

        self.resolve(&mut fun_stmt.body)?;
        self.end_scope();
        self.current_function = enclosing_function;

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

    fn visit_get_expr(&mut self, object: &Expr, _name: &Token) -> Result<Object, LoxError> {
        self.resolve_expr(object)?;

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

    fn visit_set_expr(
        &mut self,
        object: &Expr,
        _name: &Token,
        value: &Expr,
    ) -> Result<Object, LoxError> {
        self.resolve_expr(object)?;
        self.resolve_expr(value)
    }

    fn visit_this_expr(&mut self, expr: &Expr, keyword: &Token) -> Result<Object, LoxError> {
        match self.current_class {
            ClassType::None => Err(LoxError::RuntimeError(RuntimeError::new(
                "Cannot use 'this' outside of a class. -- Resolver:visit_this_expr()".to_string(),
                keyword,
            ))),
            ClassType::Class => {
                self.resolve_local(expr, keyword)?;
                Ok(Object::Nil)
            }
        }
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
        self.resolve_expr(stmt)
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
        match self.current_function {
            FunctionType::Function => self.resolve_expr(value),
            FunctionType::Initializer => {
                return Err(LoxError::RuntimeError(RuntimeError::new(
                    "Cannot return a value from an initializer. -- Resolver::visit_return_stmt()"
                        .to_string(),
                    _token,
                )))
            }
            FunctionType::Method => self.resolve_expr(value),
            FunctionType::None => {
                return Err(LoxError::RuntimeError(RuntimeError::new(
                    "Cannot return from top-level code. -- Resolver::visit_return_stmt()"
                        .to_string(),
                    _token,
                )))
            }
        }
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &mut Stmt) -> Result<Object, LoxError> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(body)
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<Object, LoxError> {
        self.declare(name);
        self.resolve_expr(initializer)?;
        self.define(name);

        Ok(Object::Nil)
    }

    fn visit_block_stmt(&mut self, statements: &mut BlockStmt) -> Result<Object, LoxError> {
        self.begin_scope();
        self.resolve(&mut statements.statements)?;
        self.end_scope();

        Ok(Object::Nil)
    }

    fn visit_function_stmt(&mut self, fun_stmt: &mut FunStmt) -> Result<Object, LoxError> {
        self.declare(&fun_stmt.name);
        self.define(&fun_stmt.name);
        self.resolve_function(fun_stmt, FunctionType::Function)?;

        Ok(Object::Nil)
    }

    fn visit_class_stmt(&mut self, class_stmt: &ClassStmt) -> Result<Object, LoxError> {
        let enclosing_class = self.current_class.clone();
        self.current_class = ClassType::Class;

        self.declare(&class_stmt.name);
        self.define(&class_stmt.name);

        self.begin_scope();
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert("this".to_string(), true);
        } else {
            return Err(LoxError::RuntimeError(RuntimeError::new(
                "Cannot resolve class scope.".to_string(),
                &class_stmt.name,
            )));
        }

        for mut method in class_stmt.methods.clone() {
            let function_type = if method.name.lexeme == "init" {
                FunctionType::Initializer
            } else {
                FunctionType::Method
            };
            match self.resolve_function(&mut method, function_type) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        self.end_scope();
        self.current_class = enclosing_class;

        Ok(Object::Nil)
    }
}
