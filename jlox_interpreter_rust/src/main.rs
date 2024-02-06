use std::{env, process};

use ast_grammar::expr::Expr;
use scanner::token::{Literal, Token, TokenType};

use crate::{ast_printer::ast_printer::AstPrinter, lox::Lox, rpn_printer::rpn_printer::RPNPrinter};

mod ast_grammar;
mod ast_printer;
mod interpreter;
mod lox;
mod parser;
mod rpn_printer;
mod scanner;

fn main() {
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
}
