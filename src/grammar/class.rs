use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::LoxError,
    grammar::{callable::LoxCallable, object::Object},
    interpreter::Interpreter,
};

use super::{callable::Callable, function::LoxFunction, instance::LoxInstance};

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

    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        match self.methods.get(name).cloned() {
            Some(method) => match method {
                Object::Callable(f) => match f {
                    Callable::LoxFunction(func) => Some(func),
                    _ => None,
                },
                _ => None,
            },
            None => None,
        }
    }
}

impl LoxCallable<Result<Object, LoxError>> for LoxClass {
    fn arity(&self) -> u8 {
        match self.find_method("init") {
            Some(method) => method.arity(),
            None => 0,
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxError> {
        let instance =
            Object::Instance(LoxInstance::new(Rc::new(RefCell::new(self.clone())))).clone();

        if let Some(initializer) = self.find_method("init") {
            initializer.call(interpreter, arguments)?;
        }
        Ok(instance)
    }
}
