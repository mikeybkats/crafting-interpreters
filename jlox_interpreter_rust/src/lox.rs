use std::{
    env, fs,
    io::{self, BufRead, BufReader, Write},
    process,
};

use crate::scanner::Scanner;

pub struct Lox {
    had_error: bool,
}
impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn main(&mut self) -> io::Result<()> {
        let args: Vec<String> = env::args().collect();

        if args.len() > 2 {
            println!("Ussage: rlox [script[");
            process::exit(64);
        } else if args.len() == 2 {
            self.run_file(&args[1])?;
        } else {
            self.run_prompt()?;
        }

        Ok(())
    }

    fn run_file(&mut self, path: &str) -> io::Result<()> {
        let bytes = fs::read(path)?;
        let content = String::from_utf8_lossy(&bytes).to_string();

        Lox::run(content);

        if self.had_error {
            process::exit(65);
        }

        Ok(())
    }

    fn run_prompt(&mut self) -> io::Result<()> {
        let input = io::stdin();
        let reader = BufReader::new(input);

        for line_result in reader.lines() {
            print!("> ");
            io::stdout().flush()?;
            let line = line_result?;

            if line.is_empty() {
                break;
            }
            Self::run(line);
            self.had_error = false;
        }

        Ok(())
    }

    fn run(source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    // fn error(&mut self, line: usize, message: &str) {
    //     self.report(line, "", message);
    // }

    // fn report(&mut self, line: usize, where_: &str, message: &str) {
    //     eprintln!("[line {}] Error{}: {}", line, where_, message);
    //     self.had_error = true;
    // }
}
