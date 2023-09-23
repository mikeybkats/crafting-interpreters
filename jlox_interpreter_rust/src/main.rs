use std::{env, process};

use crate::lox::Lox;

mod error;
mod lox;
mod scanner;
mod token;

fn main() {
    let mut lox = Lox::new();

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Ussage: rlox [script[");
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
