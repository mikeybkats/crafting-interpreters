use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{class::LoxClass, object::Object};

#[derive(Debug, Clone)]
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

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.fields.get(name) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} instance", self.class.borrow().name())
    }
}
