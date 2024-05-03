use std::fmt;

use super::{callable::Callable, instance::LoxInstance};

pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    Callable(Callable),
    Instance(LoxInstance),
    // Return(Box<Object>), // TODO: Implement Return and remove the return error type. The LoxReturn error type is not idiomatic Rust
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
            Object::Callable(_) => println!("<LoxCallable>"),
            Object::Instance(_) => println!("<LoxInstance>"),
            Object::Nil => println!("<LoxNil>"),
        }
    }

    pub fn _format(&self) -> String {
        match self {
            Object::Str(string) => format!("{string}"),
            Object::Bool(boolean) => format!("{boolean}"),
            Object::Num(number) => format!("{number}"),
            Object::Callable(_) => format!("<LoxCallable>"),
            Object::Instance(_) => format!("<LoxInstance>"),
            Object::Nil => String::from("<LoxNil>"),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Str(string) => write!(f, "{}", string),
            Object::Bool(boolean) => write!(f, "{}", boolean),
            Object::Num(number) => write!(f, "{}", number),
            Object::Instance(instance) => write!(f, "{}", instance.to_string()),
            Object::Callable(callable) => match callable {
                Callable::LoxGetter(func) => {
                    write!(f, "Object: {}", func._to_string())
                }
                Callable::LoxFunction(func) => {
                    write!(f, "Object: {}", func._to_string())
                }
                Callable::LoxClass(c) => {
                    write!(f, "{}", c.name())
                }
                Callable::LoxNativeFunction(func) => match func {
                    _ => write!(f, "Object: <LoxNativeFunction>"),
                },
            },
            Object::Nil => write!(f, "<LoxNil>"),
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Str(s) => write!(f, "Str({:?})", s),
            Object::Num(n) => write!(f, "Num({})", n),
            Object::Bool(b) => write!(f, "Bool({})", b),
            Object::Callable(callable) => match callable {
                Callable::LoxGetter(func) => {
                    return write!(f, "Object: {}", func._to_string(),);
                }
                Callable::LoxFunction(func) => {
                    return write!(f, "Object: {}", func._to_string(),);
                }
                Callable::LoxClass(c) => {
                    write!(f, "{}", c.name())
                }
                Callable::LoxNativeFunction(func) => match func {
                    _ => write!(f, "Object: <LoxNativeFunction>"),
                },
            },
            Object::Instance(i) => write!(f, "Instance({:?})", i),
            Object::Nil => write!(f, "<LoxNil>"),
        }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        match self {
            Object::Str(s) => Object::Str(s.clone()),
            Object::Num(n) => Object::Num(*n),
            Object::Bool(b) => Object::Bool(*b),
            Object::Callable(callable) => Object::Callable(callable.clone()), // Choose to return Nil for Callable
            Object::Instance(i) => Object::Instance(i.clone()),
            Object::Nil => Object::Nil,
        }
    }
}
