use std::collections::HashMap;

use crate::{ast_grammar::token::Token, error::runtime_error::RuntimeError};

pub enum EnvValue {
    _String(String),
    _Number(f64),
    _Boolean(bool),
    _Nil,
}
pub struct Environment {
    pub values: HashMap<String, EnvValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn _get_value(&self, token: &Token) -> Result<&EnvValue, RuntimeError> {
        match self.values.get(&token.lexeme) {
            Some(value) => return Ok(value),
            _ => {
                return Err(RuntimeError::new(
                    format!("Undefined variable '{}'.", token.lexeme),
                    token,
                ))
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
            .insert(token_number.lexeme.clone(), EnvValue::_Number(42.0));

        match env._get_value(&token_number) {
            Ok(EnvValue::_Number(n)) => assert_eq!(*n, 42.0),
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
            EnvValue::_String("testString".to_string()),
        );

        match env._get_value(&token_string) {
            Ok(EnvValue::_String(s)) => assert_eq!(*s, "testString"),
            _ => panic!("Value not found or not a number"),
        };

        let token_boolean = Token {
            token_type: TokenType::False,
            lexeme: "testBoolean".to_string(),
            literal: None,
            line: 1,
        };

        env.values
            .insert(token_boolean.lexeme.clone(), EnvValue::_Boolean(false));

        match env._get_value(&token_boolean) {
            Ok(EnvValue::_Boolean(b)) => assert_eq!(*b, false),
            _ => panic!("Value not found or not a number"),
        };

        let token_nil = Token {
            token_type: TokenType::Nil,
            lexeme: "testNil".to_string(),
            literal: None,
            line: 1,
        };

        env.values.insert(token_nil.lexeme.clone(), EnvValue::_Nil);

        match env._get_value(&token_nil) {
            Ok(EnvValue::_Nil) => assert!(true),
            _ => panic!("Value not found or not a number"),
        };
    }
}
