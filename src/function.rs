use std::{cell::RefCell, rc::Rc};

use crate::{
    environment,
    error::runtime_error::RuntimeError,
    grammar::{callable::LoxCallable, object::Object, stmt::FunStmt},
    interpreter::Interpreter,
};

struct LoxFunction {
    declaration: Rc<RefCell<FunStmt>>,
}

impl LoxFunction {
    pub fn _new(declaration: FunStmt) -> Self {
        Self {
            declaration: Rc::new(RefCell::new(declaration)),
        }
    }
}

impl LoxCallable<Result<Object, RuntimeError>> for LoxFunction {
    fn arity(&self) -> u8 {
        0
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
