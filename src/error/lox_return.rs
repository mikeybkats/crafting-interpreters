use crate::grammar::object::Object;

#[derive(Debug)]
pub struct LoxReturn {
    value: Option<Object>,
}

impl LoxReturn {
    pub fn new(value: Option<Object>) -> LoxReturn {
        LoxReturn { value }
    }
}

impl std::fmt::Display for LoxReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lox return statement encountered")
    }
}

impl std::error::Error for LoxReturn {}
