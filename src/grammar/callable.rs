use crate::{class::LoxClass, error::LoxError, function::LoxFunction, interpreter::Interpreter};

use super::{
    native_function::{Clock, LoxNativeFunctions},
    object::Object,
};

#[derive(Debug, Clone)]
pub enum Callable {
    LoxFunction(LoxFunction),
    LoxClass(LoxClass),
    LoxNativeFunction(LoxNativeFunctions),
}

impl Callable {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxError> {
        match self {
            Callable::LoxFunction(f) => f.call(interpreter, arguments),
            Callable::LoxClass(c) => c.call(interpreter, arguments),
            Callable::LoxNativeFunction(c) => match c {
                LoxNativeFunctions::Clock(Clock) => Clock::new().call(interpreter, vec![]),
            },
        }
    }
}

pub trait LoxCallable<T> {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> T;

    fn arity(&self) -> u8;
}
