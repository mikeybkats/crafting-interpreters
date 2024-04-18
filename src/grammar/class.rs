use std::{cell::RefCell, rc::Rc};

use crate::{
    error::LoxError,
    grammar::{callable::LoxCallable, object::Object},
    interpreter::Interpreter,
};

use super::instance::LoxInstance;

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl LoxCallable<Result<Object, LoxError>> for LoxClass {
    fn arity(&self) -> u8 {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, LoxError> {
        Ok(Object::Instance(LoxInstance::new(Rc::new(RefCell::new(
            self.clone(),
        )))))
    }
}
