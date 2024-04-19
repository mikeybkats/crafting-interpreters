use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::error::runtime_error::RuntimeError;

use super::{class::LoxClass, object::Object, token::Token};

#[derive(Debug, Clone)]
/// ## LoxInstance
/// Lox instances store state for lox classes
pub struct LoxInstance {
    class: Rc<RefCell<LoxClass>>,
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    pub fn new(class: Rc<RefCell<LoxClass>>) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        match self.fields.get(name.lexeme.as_str()) {
            Some(value) => Ok(value.clone()),
            None => Err(RuntimeError::new(
                format!("Undefined property '{}'.", name.lexeme),
                name,
            )),
        }
    }

    pub fn set(&mut self, name: &Token, value: Object) -> Option<Object> {
        self.fields.insert(name.lexeme.clone(), value)
    }

    pub fn to_string(&self) -> String {
        format!("{} instance", self.class.borrow().name())
    }
}
