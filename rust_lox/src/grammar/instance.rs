use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{environment::generate_id, error::runtime_error::RuntimeError};

use super::{
    callable::Callable, class::LoxClass, function::LoxFunction, object::Object, token::Token,
};

#[derive(Debug, Clone)]
/// ## LoxInstance
/// Lox instances store state for lox classes
pub struct LoxInstance {
    class: Rc<RefCell<LoxClass>>,
    pub fields: Rc<RefCell<HashMap<String, Object>>>,
    _id: String,
}

impl LoxInstance {
    pub fn new(class: Rc<RefCell<LoxClass>>) -> Self {
        Self {
            class,
            fields: Rc::new(RefCell::new(HashMap::new())),
            _id: generate_id(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        // if it is a field
        if let Some(value) = self.fields.clone().borrow().get(name.lexeme.as_str()) {
            return Ok(value.clone());
        }

        // if it is a method
        match self.find_method(name) {
            Some(method) => Ok(Object::Callable(Callable::LoxFunction(method))),
            // if it is not a field or method
            _ => Err(RuntimeError::new(
                format!(
                    "Undefined property '{}'. -- LoxInstance: get()",
                    name.lexeme
                ),
                name,
            )),
        }
    }

    fn find_method(&self, name: &Token) -> Option<LoxFunction> {
        let class_methods = self.class.borrow();

        if let Some(mut method) = class_methods.find_method(&name.lexeme) {
            return Some(method.bind(self.clone()));
        }

        None
    }

    pub fn set(&mut self, name: &Token, value: Object) -> Option<Object> {
        let value = self
            .fields
            .clone()
            .borrow_mut()
            .insert(name.lexeme.clone(), value);
        value
    }

    pub fn to_string(&self) -> String {
        format!("{} instance", self.class.borrow().name())
    }
}
