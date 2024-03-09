use colored::Colorize;

use crate::environment::Environment;
use crate::error::runtime_error::RuntimeError;
use crate::function::LoxFunction;
use crate::grammar::callable::Clock;
use crate::grammar::expr::{Expr, ExprVisitor};
use crate::grammar::object::Object;
use crate::grammar::stmt::{BlockStmt, FunStmt, Stmt, StmtVisitor};
use crate::grammar::token::{Token, TokenType};
use crate::lox::PromptMode;
use std::{cell::RefCell, rc::Rc};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    pub globals: Rc<RefCell<Environment>>,
    mode: PromptMode,
}

impl Interpreter {
    pub fn new() -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));

        let globals = environment.clone();

        globals.borrow_mut().define(
            "clock".to_string(),
            Object::Callable(Box::new(Clock::new())),
        );

        Self {
            globals,
            environment,
            mode: PromptMode::Single,
        }
    }

    pub fn set_mode(&mut self, mode: PromptMode) {
        self.mode = mode;
    }

    pub fn interpret(&mut self, statements: &mut Vec<Stmt>) -> Result<Vec<Object>, RuntimeError> {
        let mut results = Vec::new();
        for statement in statements {
            // println!("{} {:#?}", "statement: ".red(), statement);
            match self.execute(statement) {
                Ok(value) => results.push(value),
                Err(e) => return Err(e),
            }
        }

        Ok(results)
    }

    pub fn execute(&mut self, statement: &mut Stmt) -> Result<Object, RuntimeError> {
        match statement.accept(self) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub fn execute_block_stmt(
        &mut self,
        block_stmt: &mut BlockStmt,
        enclosed_environment: Environment,
    ) -> Result<Object, RuntimeError> {
        let previous = self.environment.clone();
        self.environment = Rc::new(RefCell::new(enclosed_environment));

        for statement in &mut block_stmt.statements {
            self.execute(statement)?;
        }

        self.environment = previous;

        Ok(Object::Nil)
    }

    pub fn evaluate(&mut self, expression: &Expr) -> Result<Object, RuntimeError> {
        match expression.accept(self) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
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

    fn check_number_operand(&self, token: &Token, operand: Object) -> Result<Object, RuntimeError> {
        match operand {
            Object::Num(_) => Ok(operand),
            _ => {
                return Err(RuntimeError::new(
                    "Operand must be a number.".to_string(),
                    token,
                ))
            }
        }
    }

    fn check_number_operands(
        &self,
        token: &Token,
        left: Object,
        right: Object,
    ) -> Result<bool, RuntimeError> {
        match (left, right) {
            (Object::Num(_left), Object::Num(_right)) => Ok(true),
            _ => {
                return Err(RuntimeError::new(
                    "Operands must be numbers.".to_string(),
                    token,
                ))
            }
        }
    }
}

impl ExprVisitor<Result<Object, RuntimeError>> for Interpreter {
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, RuntimeError> {
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
            _ => Err(RuntimeError::new(
                format!(
                    "Expression: '{} {} {}' does not evaluate.",
                    left, operator.lexeme, right,
                ),
                operator,
            )),
        }
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<Object, RuntimeError> {
        println!("callee: {:#?}", callee);
        let callee = self.evaluate(callee)?;

        let processed_arguments = arguments
            .iter()
            .map(|argument| self.evaluate(argument))
            .collect::<Result<Vec<Object>, RuntimeError>>()?;

        match callee {
            Object::Callable(function) => function.call(self, processed_arguments),
            _ => Err(RuntimeError::new(
                "Can only call functions and classes.".to_string(),
                paren,
            )),
        }
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Object, RuntimeError> {
        self.evaluate(expression)
    }

    fn visit_object_expr(&mut self, value: &Option<Object>) -> Result<Object, RuntimeError> {
        let empty_token = Token::new(TokenType::Nil, "".to_string(), Some(Object::Nil), 0);

        match value {
            Some(value) => Ok(value.clone()),
            _ => Err(RuntimeError::new("No value".to_string(), &empty_token)),
        }
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, RuntimeError> {
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

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Object, RuntimeError> {
        let right_object = self.evaluate(right)?;
        let empty_token = Token::new(TokenType::Nil, "".to_string(), Some(Object::Nil), 0);

        match (operator.token_type, right_object.clone()) {
            (TokenType::Minus, Object::Num(num)) => {
                match self.check_number_operand(operator, right_object) {
                    Ok(_result) => Ok(Object::Num(-num)),
                    Err(e) => Err(e),
                }
            }
            (TokenType::Bang, _) => Ok(Object::Bool(!right_object.is_truthy())),
            _ => Err(RuntimeError::new("No value".to_string(), &empty_token)),
        }
    }

    fn visit_variable_expr(&mut self, token: &Token) -> Result<Object, RuntimeError> {
        self.environment.borrow().get_value(token)
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Object, RuntimeError> {
        let value = self.evaluate(value)?;
        match self.environment.borrow_mut().assign(name, value.clone()) {
            Ok(_) => Ok(value),
            Err(e) => Err(e),
        }
    }
}

impl StmtVisitor<Result<Object, RuntimeError>> for Interpreter {
    fn visit_expression_stmt(&mut self, statement: &Expr) -> Result<Object, RuntimeError> {
        self.evaluate(statement)
    }

    fn visit_function_stmt(&mut self, declaration: &mut FunStmt) -> Result<Object, RuntimeError> {
        let lox_function = LoxFunction::new(declaration);
        println!("{} {:#?}", "lox_function: ".red(), lox_function);
        self.environment.borrow_mut().define(
            declaration.name.lexeme.clone(),
            Object::Callable(Box::new(lox_function)),
        );
        Ok(Object::Nil)
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &mut Stmt,
        else_branch: &mut Option<Box<Stmt>>,
    ) -> Result<Object, RuntimeError> {
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

    fn visit_print_stmt(&mut self, statement: &Expr) -> Result<Object, RuntimeError> {
        let value = self.evaluate(statement)?;
        println!("{}", value);
        Ok(Object::Nil)
    }

    fn visit_while_stmt(
        &mut self,
        condition: &Expr,
        body: &mut Stmt,
    ) -> Result<Object, RuntimeError> {
        while self.evaluate(condition)?.is_truthy() {
            match self.execute(body) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(Object::Nil)
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<Object, RuntimeError> {
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

    fn visit_block_stmt(&mut self, statements: &mut BlockStmt) -> Result<Object, RuntimeError> {
        self.execute_block_stmt(
            statements,
            Environment::with_enclosing(self.environment.clone()),
        )
    }
}
