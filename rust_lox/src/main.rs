use std::env;

use getopts::Options;

use crate::lox::Lox;

mod environment;
mod error;
mod grammar;
mod interpreter;
mod lox;
mod parser;
mod resolver;
mod scanner;

fn main() {
    let mut lox = Lox::new();

    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("", "multiline", "enable multiline mode");
    opts.optflag("", "quiet", "suppress non-program output in file mode");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string()),
    };

    let quiet_mode = matches.opt_present("quiet");
    let file_argument = matches.free.first();

    if matches.opt_present("multiline") {
        let prompt = lox.run_prompt_multiline(); // assuming you have a run_prompt_multiline method

        match prompt {
            Ok(_value) => (),
            Err(_e) => (),
        }
    } else if let Some(path) = file_argument {
        lox.run_file(path, quiet_mode).unwrap();
    } else {
        let prompt = lox.run_prompt();

        match prompt {
            Ok(_value) => (),
            Err(_e) => (),
        }
    }
}
