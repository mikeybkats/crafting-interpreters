use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::{self, Environment},
    error::LoxError,
    grammar::{
        callable::LoxCallable,
        object::Object,
        stmt::{BlockStmt, FunStmt},
    },
    interpreter::Interpreter,
};

use super::{
    instance::LoxInstance,
    stmt::{FunType, Stmt},
    token::create_this_token,
};

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: Rc<RefCell<FunStmt>>,
    closure: Rc<RefCell<environment::Environment>>,
    is_initializer: bool,
    pub is_getter: bool,
}

impl LoxFunction {
    pub fn new(
        declaration: &FunStmt,
        closure: Rc<RefCell<Environment>>,
        is_initializer: bool,
    ) -> Self {
        Self {
            declaration: Rc::new(RefCell::new(declaration.clone())),
            closure,
            is_initializer,
            is_getter: match declaration.kind {
                FunType::Getter => true,
                _ => false,
            },
        }
    }

    pub fn bind(&mut self, instance: LoxInstance) -> LoxFunction {
        let mut environment = Environment::with_enclosing(self.closure.clone());

        environment.define("this".to_string(), Object::Instance(instance));

        return LoxFunction {
            declaration: self.declaration.clone(),
            closure: Rc::new(RefCell::new(environment)),
            is_initializer: self.is_initializer,
            is_getter: self.is_getter,
        };
    }

    fn is_initializer(&self) -> bool {
        self.declaration.borrow().name.lexeme == "init"
    }

    pub fn _to_string(&self) -> String {
        format!(
            "<LoxFunction {}> -- {:#?}",
            self.declaration.borrow().name.lexeme,
            self.declaration.borrow().body
        )
    }

    pub fn _get_declaration(&self) -> Rc<RefCell<FunStmt>> {
        self.declaration.clone().to_owned()
    }

    pub fn _bound_to(&self) -> Rc<RefCell<Environment>> {
        self.closure.clone().to_owned()
    }

    fn return_val_if_initializer(
        &self,
        return_value: Result<Object, LoxError>,
    ) -> Result<Object, LoxError> {
        if self.is_initializer() {
            return match self
                .closure
                .clone()
                .borrow()
                .get_at(0, &create_this_token(None, None))
            {
                Ok(this) => Ok(this.clone()),
                Err(e) => Err(LoxError::RuntimeError(e)),
            };
        } else {
            return return_value;
        }
    }

    fn handle_block_stmt(
        &self,
        interpreter: &mut Interpreter,
        declaration_body: Vec<Stmt>,
        environment: Environment,
    ) -> Result<Object, LoxError> {
        return match interpreter.execute_block_stmt(
            &mut BlockStmt {
                statements: declaration_body,
            },
            environment,
        ) {
            Ok(value) => self.return_val_if_initializer(Ok(value)),
            Err(e) => match e {
                LoxError::RuntimeError(e) => Err(LoxError::RuntimeError(e)),
                LoxError::LoxReturn(return_value) => {
                    self.return_val_if_initializer(Err(LoxError::LoxReturn(return_value)))
                }
                LoxError::ParseError(e) => Err(LoxError::ParseError(e)),
            },
        };
    }
}

impl LoxCallable<Result<Object, LoxError>> for LoxFunction {
    fn arity(&self) -> u8 {
        self.declaration.borrow().params.len() as u8
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxError> {
        let mut environment = environment::Environment::with_enclosing(self.closure.clone());

        let dec_clone_one = self.declaration.clone();

        for (i, param) in dec_clone_one.borrow().params.iter().enumerate() {
            environment.define(param.lexeme.clone(), arguments[i].clone());
        }

        let dec_clone_two = self.declaration.clone();

        let declaration_body = dec_clone_two.borrow_mut().body.clone();

        self.handle_block_stmt(interpreter, declaration_body, environment)
    }
}
