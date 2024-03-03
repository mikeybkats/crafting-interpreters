use crate::{error::runtime_error::RuntimeError, interpreter::Interpreter};
use std::fmt;

pub trait LoxCallable {
    fn call(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>;

    fn arity(&self) -> u8;
}

/// # LoxNativeFunction
/// Crafting Interpreters 10.2 - "These are functions that the interpreter exposes to user code but that are implemented in the host language (in our case Java), not the language being implemented (Lox)."
pub struct LoxNativeFunction {
    pub arity: u8,
}
impl LoxCallable for LoxNativeFunction {
    fn arity(&self) -> u8 {
        self.arity
    }

    fn call(
        &self,
        _interpreter: &Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        // TODO: Implement LoxFunction::call
        return Ok(Object::Nil);
    }
}

pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    Callable(Box<dyn LoxCallable>),
    Nil,
}

/// # Object
///
/// The main object type for the Lox language.
///
impl Object {
    /// # is_truthy
    /// returns whether the Object value is true or false in lox
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Bool(b) => *b,
            Object::Nil => false,
            _ => true, // All other values (Str, Num) are truthy
        }
    }

    pub fn _print(&self) {
        match self {
            Object::Str(string) => println!("{string}"),
            Object::Bool(boolean) => println!("{boolean}"),
            Object::Num(number) => println!("{number}"),
            Object::Callable(_) => println!("<function>"),
            Object::Nil => println!(""),
        }
    }

    pub fn _format(&self) -> String {
        match self {
            Object::Str(string) => format!("{string}"),
            Object::Bool(boolean) => format!("{boolean}"),
            Object::Num(number) => format!("{number}"),
            Object::Callable(_) => format!("<function>"),
            Object::Nil => String::from("nil"),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Str(string) => write!(f, "{}", string),
            Object::Bool(boolean) => write!(f, "{}", boolean),
            Object::Num(number) => write!(f, "{}", number),
            Object::Callable(_) => write!(f, "<function>",),
            Object::Nil => write!(f, "nil"),
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Str(s) => write!(f, "Str({:?})", s),
            Object::Num(n) => write!(f, "Num({})", n),
            Object::Bool(b) => write!(f, "Bool({})", b),
            Object::Callable(_) => write!(f, "Callable(<function>)"),
            Object::Nil => write!(f, "Nil"),
        }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        match self {
            Object::Str(s) => Object::Str(s.clone()),
            Object::Num(n) => Object::Num(*n),
            Object::Bool(b) => Object::Bool(*b),
            Object::Callable(_) => Object::Nil, // Choose to return Nil for Callable
            Object::Nil => Object::Nil,
        }
    }
}
