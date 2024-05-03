use crate::{error::LoxError, interpreter::Interpreter};

use super::{
    class::LoxClass,
    function::LoxFunction,
    native_function::{Clock, LoxNativeFunctions},
    object::Object,
};

#[derive(Debug, Clone)]
pub enum Callable {
    LoxFunction(LoxFunction),
    LoxGetter(LoxFunction),
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
            Callable::LoxGetter(f) => f.call(interpreter, arguments),
            Callable::LoxFunction(f) => f.call(interpreter, arguments),
            Callable::LoxClass(c) => c.call(interpreter, arguments),
            Callable::LoxNativeFunction(c) => match c {
                // list of all native functions here
                LoxNativeFunctions::Clock(Clock) => Clock::new().call(interpreter, vec![]),
            },
        }
    }
}

pub trait LoxCallable<T> {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> T;

    fn arity(&self) -> u8;
}
