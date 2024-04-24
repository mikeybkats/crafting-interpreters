use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::LoxError,
    grammar::{callable::LoxCallable, object::Object},
    interpreter::Interpreter,
};

use super::instance::LoxInstance;

#[derive(Debug, Clone)]
/// ## LoxClass
/// Lox classes store the behaviors of Classes. When methods are called the data is retrieved from the LoxInstance.
pub struct LoxClass {
    name: String,
    methods: HashMap<String, Object>,
}

impl LoxClass {
    pub fn new(name: String, methods: Option<HashMap<String, Object>>) -> Self {
        let methods = match methods {
            Some(methods) => methods,
            None => HashMap::new(),
        };
        Self { name, methods }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn find_method(&self, name: &str) -> Option<Object> {
        self.methods.get(name).cloned()
    }

    pub fn bind(&mut self, instance: Object) -> LoxClass {
        unimplemented!();
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
