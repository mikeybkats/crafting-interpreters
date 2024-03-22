use std::{cell::RefCell, rc::Rc};

use crate::{
    environment,
    error::LoxError,
    grammar::{callable::LoxCallable, object::Object, stmt::FunStmt},
    interpreter::Interpreter,
};

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: Rc<RefCell<FunStmt>>,
}

impl LoxFunction {
    pub fn new(declaration: &FunStmt) -> Self {
        Self {
            declaration: Rc::new(RefCell::new(declaration.clone())),
        }
    }

    pub fn _to_string(&self) -> String {
        format!("<fn {}>", self.declaration.borrow().name.lexeme)
    }
}

impl LoxCallable<Result<Object, LoxError>> for LoxFunction {
    fn arity(&self) -> u8 {
        self.declaration.borrow().params.len() as u8
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxError> {
        let mut environment = environment::Environment::with_enclosing(interpreter.globals.clone());

        for (i, param) in self.declaration.borrow().params.iter().enumerate() {
            environment.define(param.lexeme.clone(), arguments[i].clone());
        }

        return match interpreter
            .execute_block_stmt(&mut self.declaration.borrow_mut().body, environment)
        {
            Ok(value) => Ok(value),
            Err(e) => match e {
                LoxError::RuntimeError(e) => Err(LoxError::RuntimeError(e)),
                LoxError::LoxReturn(return_value) => Err(LoxError::LoxReturn(return_value)),
                LoxError::ParseError(e) => Err(LoxError::ParseError(e)),
            },
        };
    }
}
