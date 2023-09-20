use std::{
    env, fs,
    io::{self, BufRead, BufReader, Write},
    process,
};

use crate::scanner::Scanner;

pub struct Lox;
impl Lox {
    pub fn main() -> io::Result<()> {
        let args: Vec<String> = env::args().collect();

        if args.len() > 2 {
            println!("Ussage: rlox [script[");
            process::exit(64);
        } else if args.len() == 2 {
            Lox::run_file(&args[1])?;
        } else {
            Lox::run_prompt()?;
        }

        Ok(())
    }

    fn run_file(path: &str) -> io::Result<()> {
        let bytes = fs::read(path)?;
        let content = String::from_utf8_lossy(&bytes).to_string();

        Lox::run(content);

        Ok(())
    }

    fn run_prompt() -> io::Result<()> {
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
        }

        Ok(())
    }

    fn run(source: String) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("{:?}", token);
        }
    }
}
