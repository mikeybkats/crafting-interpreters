use std::{cell::RefCell, env, rc::Rc};

use crate::{
    environment::{self, Environment},
    error::LoxError,
    grammar::{
        callable::LoxCallable,
        object::Object,
        stmt::{BlockStmt, FunStmt},
    },
    interpreter::Interpreter,
};

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: Rc<RefCell<FunStmt>>,
    closure: Rc<RefCell<environment::Environment>>,
}

impl LoxFunction {
    pub fn new(declaration: &FunStmt, closure: Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration: Rc::new(RefCell::new(declaration.clone())),
            closure,
        }
    }

    pub fn _to_string(&self) -> String {
        format!("<fn {}>", self.declaration.borrow().name.lexeme)
    }

    pub fn bind(&mut self, instance: Object) -> LoxFunction {
        let mut environment = Environment::with_enclosing(self.closure.clone());
        environment.define("this".to_string(), instance);

        return LoxFunction {
            declaration: self.declaration.clone(),
            closure: Rc::new(RefCell::new(environment)),
        };
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
        let mut environment = environment::Environment::with_enclosing(self.closure.clone());

        let dec_clone_one = self.declaration.clone();

        for (i, param) in dec_clone_one.borrow().params.iter().enumerate() {
            environment.define(param.lexeme.clone(), arguments[i].clone());
        }

        let dec_clone_two = self.declaration.clone();

        let declaration_body = dec_clone_two.borrow_mut().body.clone();
        return match interpreter.execute_block_stmt(
            &mut BlockStmt {
                statements: declaration_body,
            },
            environment,
        ) {
            Ok(value) => Ok(value),
            Err(e) => match e {
                LoxError::RuntimeError(e) => Err(LoxError::RuntimeError(e)),
                LoxError::LoxReturn(return_value) => Err(LoxError::LoxReturn(return_value)),
                LoxError::ParseError(e) => Err(LoxError::ParseError(e)),
            },
        };
    }
}
