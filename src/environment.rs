use std::collections::HashMap;

use crate::{
    ast_grammar::token::{Literal, Token},
    error::runtime_error::RuntimeError,
};

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, Literal>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    // Secondary constructor: With an enclosing environment
    pub fn with_enclosing(enclosing: Environment) -> Self {
        Environment {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn assign(&mut self, name: &Token, value: Literal) -> Result<Literal, RuntimeError> {
        match self.values.get_mut(&name.lexeme) {
            Some(v) => {
                *v = value;
                Ok(v.clone())
            }
            _ => {
                if self.enclosing.is_some() {
                    return self
                        .enclosing
                        .as_mut()
                        .unwrap_or_else(|| panic!("Enclosing environment is None"))
                        .assign(name, value);
                } else {
                    return Err(RuntimeError::new(
                        format!("Undefined variable '{}'.", name.lexeme),
                        name,
                    ));
                }
            }
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get_value(&self, token: &Token) -> Result<Literal, RuntimeError> {
        match self.values.get(&token.lexeme) {
            Some(value) => return Ok(value.clone()),
            _ => {
                if self.enclosing.is_some() {
                    return self
                        .enclosing
                        .as_ref()
                        .unwrap_or_else(|| panic!("Enclosing environment is None"))
                        .get_value(token);
                } else {
                    return Err(RuntimeError::new(
                        format!("Undefined variable '{}'.", token.lexeme),
                        token,
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_grammar::token::Token;
    use crate::ast_grammar::token::TokenType;

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
            .insert(token_number.lexeme.clone(), Literal::Num(42.0));

        match env.get_value(&token_number) {
            Ok(Literal::Num(n)) => assert_eq!(n, 42.0),
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
            Literal::Str("testString".to_string()),
        );

        match env.get_value(&token_string) {
            Ok(Literal::Str(s)) => assert_eq!(s, "testString"),
            _ => panic!("Value not found or not a number"),
        };

        let token_boolean = Token {
            token_type: TokenType::False,
            lexeme: "testBoolean".to_string(),
            literal: None,
            line: 1,
        };

        env.values
            .insert(token_boolean.lexeme.clone(), Literal::Bool(false));

        match env.get_value(&token_boolean) {
            Ok(Literal::Bool(b)) => assert_eq!(b, false),
            _ => panic!("Value not found or not a number"),
        };

        let token_nil = Token {
            token_type: TokenType::Nil,
            lexeme: "testNil".to_string(),
            literal: None,
            line: 1,
        };

        env.values.insert(token_nil.lexeme.clone(), Literal::Nil);

        match env.get_value(&token_nil) {
            Ok(Literal::Nil) => assert!(true),
            _ => panic!("Value not found or not a number"),
        };
    }
}
