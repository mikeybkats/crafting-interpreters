use std::{cell::RefCell, rc::Rc};

use crate::class::LoxClass;

#[derive(Debug, Clone)]
pub struct LoxInstance {
    class: Rc<RefCell<LoxClass>>,
}

impl LoxInstance {
    pub fn new(class: Rc<RefCell<LoxClass>>) -> Self {
        Self { class }
    }

    pub fn to_string(&self) -> String {
        format!("{} instance", self.class.borrow().name())
    }
}
