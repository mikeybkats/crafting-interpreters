use std::{env, process};

use expr::{AstPrinter, Expr};
use token::{StringOrNumber, Token, TokenType};

use crate::lox::Lox;

mod error;
mod expr;
mod lox;
mod scanner;
mod token;

fn main() {
    let expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new(Expr::Literal {
                value: Some(StringOrNumber::Num(123.0)),
            }),
        }),
        operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Some(StringOrNumber::Num(45.67)),
            }),
        }),
    };

    let result = AstPrinter::print(expr);
    println!("{}", result);
}

fn main2() {
    let mut lox = Lox::new();

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script[");
        process::exit(64);
    } else if args.len() == 2 {
        lox.run_file(&args[1]).unwrap();
    } else {
        let prompt = lox.run_prompt();

        match prompt {
            Ok(_value) => (),
            Err(_e) => (),
        }
    }

    // Ok(())
}
