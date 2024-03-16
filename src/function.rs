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
    pub fn new(declaration: &mut FunStmt) -> Self {
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
        let mut environment = environment::Environment::new();
        environment.enclosing = Some(interpreter.globals.clone());

        let dec_clone_one = self.declaration.clone();

        for (i, param) in dec_clone_one.borrow().params.iter().enumerate() {
            environment.define(param.lexeme.clone(), arguments[i].clone());
        }

        let dec_clone_two = self.declaration.clone();

        let mut declaration_body = dec_clone_two.borrow_mut().body.clone();
        return match interpreter.execute_block_stmt(&mut declaration_body, environment) {
            Ok(value) => Ok(value),
            Err(e) => match e {
                LoxError::RuntimeError(e) => Err(LoxError::RuntimeError(e)),
                LoxError::LoxReturn(return_value) => {
                    // println!("LoxFunction::call - LoxReturn: {:?}", return_value);
                    return Ok(return_value.value.unwrap_or(Object::Nil));
                }
                LoxError::ParseError(e) => Err(LoxError::ParseError(e)),
            },
        };
    }
}
