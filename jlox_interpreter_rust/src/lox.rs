use std::{
    cell::RefCell,
    env, fs,
    io::{self, BufRead, BufReader, Write},
    process,
    rc::Rc,
};

use crate::{error::ErrorReporter, scanner::Scanner};

pub struct Lox {
    error_reporter: Rc<RefCell<ErrorReporter>>,
}
impl Lox {
    pub fn new() -> Self {
        Self {
            // use reference counter to count references for any sub impl that will need to report errors
            error_reporter: Rc::new(RefCell::new(ErrorReporter::new())),
        }
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

        self.run(content);

        if self.error_reporter.borrow_mut().had_error() {
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
            self.run(line);
            self.error_reporter.borrow_mut().set_error(false);
        }

        Ok(())
    }

    fn run(&self, source: String) {
        let mut scanner = Scanner::new(source, Rc::clone(&self.error_reporter));
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }
}
