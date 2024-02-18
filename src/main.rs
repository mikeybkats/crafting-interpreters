use std::env;

use getopts::Options;

use crate::lox::Lox;

mod ast_grammar;
mod ast_printer;
mod environment;
mod error;
mod interpreter;
mod lox;
mod parser;
mod rpn_printer;
mod scanner;

fn main() {
    let mut lox = Lox::new();

    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("", "multiline", "enable multiline mode");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string()),
    };

    if matches.opt_present("multiline") {
        let prompt = lox.run_prompt_multiline(); // assuming you have a run_prompt_multiline method

        match prompt {
            Ok(_value) => (),
            Err(_e) => (),
        }
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
