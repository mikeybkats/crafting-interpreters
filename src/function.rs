use std::{cell::RefCell, rc::Rc};

use crate::{
    environment,
    error::runtime_error::RuntimeError,
    grammar::{callable::LoxCallable, object::Object, stmt::FunStmt},
    interpreter::Interpreter,
};

#[derive(Debug)]
pub struct LoxFunction {
    declaration: Rc<RefCell<FunStmt>>,
}

impl LoxFunction {
    pub fn new(declaration: &mut FunStmt) -> Self {
        Self {
            declaration: Rc::new(RefCell::new(declaration.clone())),
        }
    }

    // pub fn arity(&self) -> u8 {
    //     self.declaration.borrow().params.len() as u8
    // }

    pub fn _to_string(&self) -> String {
        format!("<fn {}>", self.declaration.borrow().name.lexeme)
    }
}

impl LoxCallable<Result<Object, RuntimeError>> for LoxFunction {
    fn arity(&self) -> u8 {
        self.declaration.borrow().params.len() as u8
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let mut environment = environment::Environment::new();
        environment.enclosing = Some(interpreter.globals.clone());

        for (i, param) in self.declaration.borrow_mut().params.iter().enumerate() {
            environment.define(param.lexeme.clone(), arguments[i].clone());
        }

        interpreter.execute_block_stmt(&mut self.declaration.borrow_mut().body, environment)
    }
}