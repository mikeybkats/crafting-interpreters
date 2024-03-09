use crate::{error::runtime_error::RuntimeError, interpreter::Interpreter};

use super::object::Object;
pub trait LoxCallable<T> {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> T;

    fn arity(&self) -> u8;
}

/// # LoxNativeFunction
/// Crafting Interpreters 10.2 - "These are functions that the interpreter exposes to user code but that are implemented in the host language (in our case Java), not the language being implemented (Lox)."
pub struct LoxNativeFunction {
    pub arity: u8,
}
impl LoxCallable<Option<Object>> for LoxNativeFunction {
    fn arity(&self) -> u8 {
        self.arity
    }

    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Option<Object> {
        // TODO: Implement LoxFunction::call
        None
    }
}

pub struct Clock;
impl Clock {
    pub fn new() -> Self {
        Self {}
    }

    pub fn _string(&self) -> String {
        String::from("<native fn>")
    }
}
impl LoxCallable<Result<Object, RuntimeError>> for Clock {
    fn arity(&self) -> u8 {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        Ok(Object::Num(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        ))
    }
}
