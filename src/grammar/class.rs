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
    superclass: Option<Box<LoxClass>>,
}

impl LoxClass {
    pub fn new(
        name: String,
        superclass: Option<LoxClass>,
        methods: Option<HashMap<String, Object>>,
    ) -> Self {
        let methods = match methods {
            Some(methods) => methods,
            None => HashMap::new(),
        };
        let superclass: Option<Box<LoxClass>> = match superclass {
            Some(superclass) => Some(Box::new(superclass)),
            None => None,
        };

        Self {
            name,
            methods,
            superclass,
        }
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
            None => {
                if let Some(superclass) = self.superclass.clone() {
                    superclass.find_method(name)
                } else {
                    None
                }
            }
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
        let instance = LoxInstance::new(Rc::new(RefCell::new(self.clone())));

        if let Some(mut initializer) = self.find_method("init") {
            initializer.bind(instance).call(interpreter, arguments)
        } else {
            Ok(Object::Instance(instance).clone())
        }
    }
}
