use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::runtime_error::RuntimeError,
    grammar::{object::Object, token::Token},
};

use rand::distributions::Alphanumeric;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::{SystemTime, UNIX_EPOCH};

fn generate_id() -> String {
    // Obtain the current system time as a seed
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();

    // Initialize a random number generator with the current system time as the seed
    let rng = StdRng::seed_from_u64(seed as u64);

    // Generate a random string of 5 alphanumeric characters
    let id: String = rng
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    id
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, Object>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub name: String,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            name: generate_id(),
            enclosing: None,
            values: HashMap::new(),
        }
    }

    // Secondary constructor: With an enclosing environment
    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            name: format!("{}-child", generate_id()),
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<Object, RuntimeError> {
        match self.values.get_mut(&name.lexeme) {
            Some(v) => {
                *v = value;
                Ok(v.clone())
            }
            _ => {
                // if the env has an enclosing env, see if the name is there
                if self.enclosing.is_some() {
                    match self.enclosing.as_mut() {
                        Some(env) => env.borrow_mut().assign(name, value),
                        _ => Err(RuntimeError::new(
                            format!("Undefined variable '{}'.", name.lexeme),
                            name,
                        )),
                    }
                } else {
                    return Err(RuntimeError::new(
                        format!("Undefined variable '{}'.", name.lexeme),
                        name,
                    ));
                }
            }
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn _values_of(&self) -> &HashMap<String, Object> {
        &self.values
    }

    pub fn get_value(&self, token: &Token) -> Result<Object, RuntimeError> {
        match self.values.get(&token.lexeme) {
            Some(value) => Ok(value.clone()),
            _ => match self.enclosing.as_deref() {
                Some(env) => env.borrow().get_value(token),
                None => Err(RuntimeError::new(
                    format!("Undefined variable '{}'.", token.lexeme),
                    token,
                )),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::token::Token;
    use crate::grammar::token::TokenType;

    #[test]
    fn test_new_environment() {
        let env = Environment::new();
        assert!(env.values.is_empty(), "New environment should be empty");
    }

    #[test]
    fn test_get_value() {
        let mut env = Environment::new();

        let token_number = Token {
            token_type: TokenType::Number,
            lexeme: "testNumber".to_string(),
            literal: None,
            line: 1,
        };

        env.values
            .insert(token_number.lexeme.clone(), Object::Num(42.0));

        match env.get_value(&token_number) {
            Ok(Object::Num(n)) => assert_eq!(n, 42.0),
            _ => panic!("Value not found or not a number"),
        };

        let token_string = Token {
            token_type: TokenType::String,
            lexeme: "testString".to_string(),
            literal: None,
            line: 1,
        };

        env.values.insert(
            token_string.lexeme.clone(),
            Object::Str("testString".to_string()),
        );

        match env.get_value(&token_string) {
            Ok(Object::Str(s)) => assert_eq!(s, "testString"),
            _ => panic!("Value not found or not a number"),
        };

        let token_boolean = Token {
            token_type: TokenType::False,
            lexeme: "testBoolean".to_string(),
            literal: None,
            line: 1,
        };

        env.values
            .insert(token_boolean.lexeme.clone(), Object::Bool(false));

        match env.get_value(&token_boolean) {
            Ok(Object::Bool(b)) => assert_eq!(b, false),
            _ => panic!("Value not found or not a number"),
        };

        let token_nil = Token {
            token_type: TokenType::Nil,
            lexeme: "testNil".to_string(),
            literal: None,
            line: 1,
        };

        env.values.insert(token_nil.lexeme.clone(), Object::Nil);

        match env.get_value(&token_nil) {
            Ok(Object::Nil) => assert!(true),
            _ => panic!("Value not found or not a number"),
        };
    }
}
