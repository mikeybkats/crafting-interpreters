use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::error::runtime_error::RuntimeError;

use super::{callable::Callable, class::LoxClass, object::Object, token::Token};

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
            None => {
                if let Some(method) = self.class.borrow().find_method(&name.lexeme) {
                    match method {
                        Object::Callable(callable) => {
                            let mut callable = callable.clone();

                            match callable.bind(Object::Instance(self.clone())) {
                                Callable::LoxFunction(func) => {
                                    return Ok(Object::Callable(Callable::LoxFunction(func)))
                                }
                                _ => {
                                    return Err(RuntimeError::new(
                                        "Cannot bind non-callable object to instance.".to_string(),
                                        name,
                                    ))
                                }
                            }
                        }
                        _ => return Ok(method),
                    }
                } else {
                    Err(RuntimeError::new(
                        format!("Undefined property '{}'.", name.lexeme),
                        name,
                    ))
                }
            }
        }
    }

    fn bind_method(&self, name: &Token) -> Result<Object, RuntimeError> {
        unimplemented!()
    }

    fn find_and_bind_method(&self, name: &Token) -> Result<Object, RuntimeError> {
        unimplemented!()
    }

    pub fn set(&mut self, name: &Token, value: Object) -> Option<Object> {
        self.fields.insert(name.lexeme.clone(), value)
    }

    pub fn to_string(&self) -> String {
        format!("{} instance", self.class.borrow().name())
    }
}
