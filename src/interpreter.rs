use crate::environment::{generate_id, Environment};
use crate::error::lox_return::LoxReturn;
use crate::error::runtime_error::RuntimeError;
use crate::error::LoxError;
use crate::grammar::callable::{Callable, LoxCallable};
use crate::grammar::class::LoxClass;
use crate::grammar::expr::{Expr, ExprVisitor};
use crate::grammar::function::LoxFunction;
use crate::grammar::native_function::{Clock, LoxNativeFunctions};
use crate::grammar::object::Object;
use crate::grammar::stmt::{BlockStmt, ClassStmt, FunStmt, Stmt, StmtVisitor};
use crate::grammar::token::{Token, TokenType};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

/// # Interpreter
pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    pub globals: Rc<RefCell<Environment>>,
    /// Locals stores the distance of a variable from the current scope. A given expression is so many scopes away from the current scope
    // TODO: convert locals to a vector. Look up values by index, which needs saved the scopes vector in Resolver
    locals: Rc<RefCell<HashMap<Expr, usize>>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));

        let globals = environment.clone();

        globals.borrow_mut().define(
            "clock".to_string(),
            Object::Callable(Callable::LoxNativeFunction(LoxNativeFunctions::Clock(
                Clock::new(),
            ))),
        );

        Self {
            globals,
            environment,
            locals: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn interpret(&mut self, statements: &mut Vec<Stmt>) -> Result<Vec<Object>, LoxError> {
        let mut results = Vec::new();
        // println!("Interpreting -- statements: {:#?}", statements);
        for statement in statements {
            match self.execute(statement) {
                Ok(value) => results.push(value),
                Err(e) => return Err(e),
            }
        }

        Ok(results)
    }

    pub fn execute(&mut self, statement: &mut Stmt) -> Result<Object, LoxError> {
        match statement.accept(self) {
            Ok(value) => return Ok(value),
            Err(e) => return Err(e),
        }
    }

    pub fn resolve(&mut self, expr: &Expr, depth: usize) -> Result<Object, LoxError> {
        self.locals.borrow_mut().insert(expr.clone(), depth);

        Ok(Object::Nil)
    }

    pub fn execute_block_stmt(
        &mut self,
        block_stmt: &mut BlockStmt,
        enclosed_environment: Environment,
    ) -> Result<Object, LoxError> {
        let previous = self.environment.clone();

        self.environment = Rc::new(RefCell::new(enclosed_environment));

        for statement in &mut block_stmt.statements {
            match self.execute(statement) {
                Ok(_) => {}
                Err(e) => match e {
                    LoxError::LoxReturn(return_value) => {
                        self.environment = previous;
                        return Ok(return_value.value.unwrap_or(Object::Nil));
                    }
                    _ => {
                        return Err(e);
                    }
                },
            }
        }

        self.environment = previous;

        Ok(Object::Nil)
    }

    pub fn evaluate(&mut self, expression: &Expr) -> Result<Object, LoxError> {
        match expression.accept(self) {
            Ok(value) => return Ok(value),
            Err(e) => return Err(e),
        }
    }

    fn is_equal(&self, a: &Object, b: &Object) -> bool {
        match (a, b) {
            (Object::Num(a), Object::Num(b)) => a == b,
            (Object::Str(a), Object::Str(b)) => a == b,
            (Object::Bool(a), Object::Bool(b)) => a == b,
            _ => false,
        }
    }

    fn check_number_operand(&self, token: &Token, operand: Object) -> Result<Object, LoxError> {
        match operand {
            Object::Num(_) => Ok(operand),
            _ => Err(LoxError::RuntimeError(RuntimeError::new(
                "Operand must be a number.".to_string(),
                token,
            ))),
        }
    }

    fn check_number_operands(
        &self,
        token: &Token,
        left: Object,
        right: Object,
    ) -> Result<bool, LoxError> {
        match (left, right) {
            (Object::Num(_left), Object::Num(_right)) => Ok(true),
            _ => Err(LoxError::RuntimeError(RuntimeError::new(
                "Operands must be numbers.".to_string(),
                token,
            ))),
        }
    }

    fn look_up_variable(&self, name: &Token, expr: &Expr) -> Result<Object, LoxError> {
        let locals = self.locals.borrow();
        let distance = locals.get(&expr);

        match distance {
            Some(distance) => {
                let value = self.environment.borrow().get_at(*distance, name);
                match value {
                    Ok(value) => Ok(value),
                    Err(e) => Err(LoxError::RuntimeError(e)),
                }
            }
            None => {
                let value = self.globals.borrow().get_value(name);
                match value {
                    Ok(value) => Ok(value),
                    Err(e) => Err(LoxError::RuntimeError(e)),
                }
            }
        }
    }
}

impl ExprVisitor<Result<Object, LoxError>> for Interpreter {
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, LoxError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match (operator.token_type, left.clone(), right.clone()) {
            // Handle equals
            (TokenType::BangEqual, left_num, right_num) => {
                Ok(Object::Bool(!self.is_equal(&left_num, &right_num)))
            }
            (TokenType::EqualEqual, left_num, right_num) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Object::Bool(self.is_equal(&left_num, &right_num))),
                    Err(e) => Err(e),
                }
            }

            // Handle greater than
            (TokenType::Greater, Object::Num(left_num), Object::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Object::Bool(left_num > right_num)),
                    Err(e) => {
                        // TODO: handle error
                        Err(e)
                    }
                }
            }
            (TokenType::GreaterEqual, Object::Num(left_num), Object::Num(right_num)) => {
                Ok(Object::Bool(left_num >= right_num))
            }

            // Handle less than
            (TokenType::Less, Object::Num(left_num), Object::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Object::Bool(left_num < right_num)),
                    Err(e) => Err(e),
                }
            }
            (TokenType::LessEqual, Object::Num(left_num), Object::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Object::Bool(left_num <= right_num)),
                    Err(e) => Err(e),
                }
            }

            // Handle subtraction
            (TokenType::Minus, Object::Num(left_num), Object::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_result) => Ok(Object::Num(left_num - right_num)),
                    Err(e) => Err(e),
                }
            }

            // Handle addition
            (TokenType::Plus, Object::Num(left_num), Object::Num(right_num)) => {
                Ok(Object::Num(left_num + right_num))
            }
            (TokenType::Plus, Object::Str(left_str), Object::Str(right_str)) => {
                Ok(Object::Str(format!("{}{}", left_str, right_str)))
            }

            // Handle addition of number and string concatenation
            (TokenType::Plus, Object::Num(left_num), Object::Str(right_str)) => {
                Ok(Object::Str(format!("{}{}", left_num, right_str)))
            }

            // Handle addition of string and number concatenation
            (TokenType::Plus, Object::Str(left_str), Object::Num(right_num)) => {
                Ok(Object::Str(format!("{}{}", left_str, right_num)))
            }

            // Handle division
            (TokenType::Slash, Object::Num(left_num), Object::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Object::Num(left_num / right_num)),
                    Err(e) => Err(e),
                }
            }

            // Handle multiplication
            (TokenType::Star, Object::Num(left_num), Object::Num(right_num)) => {
                match self.check_number_operands(operator, left, right) {
                    Ok(_) => Ok(Object::Num(left_num * right_num)),
                    Err(e) => Err(e),
                }
            }

            // Handle errors
            _ => Err(LoxError::RuntimeError(RuntimeError::new(
                format!(
                    "Expression: '{} {} {}' does not evaluate.",
                    left, operator.lexeme, right,
                ),
                operator,
            ))),
        }
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<Object, LoxError> {
        let processed_callee = self.evaluate(callee)?;

        let processed_arguments = arguments
            .iter()
            .map(|argument| self.evaluate(argument))
            .collect::<Result<Vec<Object>, LoxError>>()?;

        match processed_callee {
            Object::Callable(function) => function.call(self, processed_arguments),
            _ => Err(LoxError::RuntimeError(RuntimeError::new(
                "Can only call functions and classes. -- Interpreter: visit_call_expr()"
                    .to_string(),
                paren,
            ))),
        }
    }

    fn visit_get_expr(&mut self, object: &Expr, name: &Token) -> Result<Object, LoxError> {
        let object = self.evaluate(object)?;

        match object {
            Object::Instance(instance) => match instance.get(&name) {
                Ok(value) => {
                    match value {
                        Object::Callable(callable) => match callable {
                            Callable::LoxGetter(getter) => getter.call(self, Vec::new()),
                            _ => Ok(Object::Callable(callable)),
                        },
                        _ => Ok(value),
                    }
                    // Ok(value)
                }
                Err(e) => Err(LoxError::RuntimeError(e)),
            },
            _ => Err(LoxError::RuntimeError(RuntimeError::new(
                "Only instances have properties. -- Interpreter: visit_get_expr()".to_string(),
                name,
            ))),
        }
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Object, LoxError> {
        self.evaluate(expression)
    }

    fn visit_literal_expr(&mut self, value: &Option<Object>) -> Result<Object, LoxError> {
        let empty_token = Token::new(
            TokenType::Nil,
            "".to_string(),
            Some(Object::Nil),
            0,
            generate_id(),
        );

        match value {
            Some(value) => Ok(value.clone()),
            _ => Err(LoxError::RuntimeError(RuntimeError::new(
                "No value".to_string(),
                &empty_token,
            ))),
        }
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, LoxError> {
        let left = self.evaluate(left)?;

        if operator.token_type == TokenType::Or {
            if left.is_truthy() {
                return Ok(left);
            }
        } else {
            if !left.is_truthy() {
                return Ok(left);
            }
        }

        self.evaluate(right)
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Object, LoxError> {
        let right_object = self.evaluate(right)?;
        let empty_token = Token::new(
            TokenType::Nil,
            "".to_string(),
            Some(Object::Nil),
            0,
            generate_id(),
        );

        match (operator.token_type, right_object.clone()) {
            (TokenType::Minus, Object::Num(num)) => {
                match self.check_number_operand(operator, right_object) {
                    Ok(_result) => Ok(Object::Num(-num)),
                    Err(e) => Err(e),
                }
            }
            (TokenType::Bang, _) => Ok(Object::Bool(!right_object.is_truthy())),
            _ => Err(LoxError::RuntimeError(RuntimeError::new(
                "No value".to_string(),
                &empty_token,
            ))),
        }
    }

    fn visit_set_expr(
        &mut self,
        object: &Expr,
        name: &Token,
        value: &Expr,
    ) -> Result<Object, LoxError> {
        let object = self.evaluate(object)?;

        match object {
            Object::Instance(mut instance) => {
                let value_obj = self.evaluate(value)?;
                match instance.set(name, value_obj) {
                    Some(return_val) => Ok(return_val),
                    None => Ok(Object::Nil),
                }
            }
            _ => {
                return Err(LoxError::RuntimeError(RuntimeError::new(
                    "Only instances have fields.".to_string(),
                    name,
                )))
            }
        }
    }

    fn visit_this_expr(&mut self, expr: &Expr, keyword: &Token) -> Result<Object, LoxError> {
        return self.look_up_variable(keyword, expr);
    }

    /// ## visit_assign_expr
    /// Runs only on resassignment
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Object, LoxError> {
        let value_obj = self.evaluate(value)?;

        let locals = self.locals.borrow();
        let distance = locals.get(&value);
        match distance {
            Some(distance) => self
                .environment
                .borrow_mut()
                .assign_at(*distance, name, value_obj)
                .or_else(|error| Err(LoxError::RuntimeError(error))),
            None => self
                .globals
                .borrow_mut()
                .assign(name, value_obj)
                .or_else(|error| Err(LoxError::RuntimeError(error))),
        }
    }

    fn visit_variable_expr(&mut self, expr: &Expr, name: &Token) -> Result<Object, LoxError> {
        self.look_up_variable(name, expr)
    }
}

impl StmtVisitor<Result<Object, LoxError>> for Interpreter {
    fn visit_expression_stmt(&mut self, statement: &Expr) -> Result<Object, LoxError> {
        self.evaluate(statement)
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &mut Stmt,
        else_branch: &mut Option<Box<Stmt>>,
    ) -> Result<Object, LoxError> {
        match self.evaluate(condition) {
            Ok(value) => {
                if value.is_truthy() {
                    return self.execute(then_branch);
                } else if let Some(else_branch) = else_branch {
                    return self.execute(else_branch);
                }
            }
            Err(e) => return Err(e),
        }
        Ok(Object::Nil)
    }

    fn visit_print_stmt(&mut self, statement: &Expr) -> Result<Object, LoxError> {
        let value = self.evaluate(statement)?;
        println!("{}", value);
        Ok(Object::Nil)
    }

    fn visit_return_stmt(&mut self, _token: &Token, value: &Expr) -> Result<Object, LoxError> {
        let value = self.evaluate(value)?;

        // throw an error to trigger an escape from deep call stack
        Err(LoxError::LoxReturn(LoxReturn::new(Some(value))))
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &mut Stmt) -> Result<Object, LoxError> {
        while self.evaluate(condition)?.is_truthy() {
            match self.execute(body) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(Object::Nil)
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<Object, LoxError> {
        match self.evaluate(initializer) {
            Ok(value) => {
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), value.clone());

                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    fn visit_block_stmt(&mut self, statements: &mut BlockStmt) -> Result<Object, LoxError> {
        self.execute_block_stmt(
            statements,
            Environment::with_enclosing(self.environment.clone()),
        )
    }

    fn visit_function_stmt(&mut self, declaration: &mut FunStmt) -> Result<Object, LoxError> {
        let lox_function = LoxFunction::new(declaration, self.environment.clone(), false);
        self.environment.borrow_mut().define(
            declaration.name.lexeme.clone(),
            Object::Callable(Callable::LoxFunction(lox_function)),
        );

        Ok(Object::Nil)
    }

    fn visit_getter_stmt(&mut self, declaration: &mut FunStmt) -> Result<Object, LoxError> {
        // define the getter function in the environment
        let lox_function = LoxFunction::new(declaration, self.environment.clone(), false);
        self.environment.borrow_mut().define(
            declaration.name.lexeme.clone(),
            Object::Callable(Callable::LoxFunction(lox_function)),
        );

        Ok(Object::Nil)
    }

    fn visit_class_stmt(&mut self, class_stmt: &ClassStmt) -> Result<Object, LoxError> {
        self.environment
            .borrow_mut()
            .define(class_stmt.name.lexeme.clone(), Object::Nil);

        let mut methods = HashMap::new();

        for method in class_stmt.methods.clone() {
            let lox_function = LoxFunction::new(
                &method,
                self.environment.clone(),
                method.name.lexeme == "init",
            );
            methods.insert(
                method.name.lexeme.clone(),
                Object::Callable(Callable::LoxFunction(lox_function)),
            );
        }

        let class = LoxClass::new(class_stmt.name.lexeme.clone(), Some(methods));

        let assignment = self.environment.borrow_mut().assign(
            &class_stmt.name,
            Object::Callable(Callable::LoxClass(class)),
        );

        match assignment {
            Ok(_) => Ok(Object::Nil),
            Err(e) => Err(LoxError::RuntimeError(e)),
        }
    }
}
