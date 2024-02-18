use std::io::{BufRead, Read, Write};
use std::{
    cell::RefCell,
    fs,
    io::{self},
    process,
    rc::Rc,
};

use crate::{
    // ast_grammar::token::Literal,
    error::{error::ErrorReporter, parse_error::ParseError, runtime_error::RuntimeError},
    interpreter::Interpreter,
    parser::Parser,
    scanner::Scanner,
};

pub enum LoxError {
    RuntimeError(RuntimeError),
    ParseError(ParseError),
}

pub struct Lox {
    error_reporter: Rc<RefCell<ErrorReporter>>,
    interpreter: Rc<RefCell<Interpreter>>,
}
impl Lox {
    pub fn new() -> Self {
        Self {
            // use reference counter to count references for any sub impl that will need to report errors
            error_reporter: Rc::new(RefCell::new(ErrorReporter::new())),
            interpreter: Rc::new(RefCell::new(Interpreter::new())),
        }
    }

    pub fn error(&self, error: LoxError) {
        match error {
            LoxError::RuntimeError(error) => {
                self.error_reporter.borrow_mut().report_runtime_error(error)
            }
            LoxError::ParseError(error) => {
                self.error_reporter.borrow_mut().report_parse_error(error)
            }
        }
    }

    pub fn run_file(&mut self, path: &str) -> io::Result<()> {
        println!("\n");
        println!("--------------");
        println!("Welcome to Lox");
        println!("--------------");
        println!("\n");
        println!("running file {} \n\n", path);

        let bytes = fs::read(path)?;
        let content = String::from_utf8_lossy(&bytes).to_string();

        let _value_of_run = self.run(content);

        if self.error_reporter.borrow_mut().had_error() {
            process::exit(65);
        }

        Ok(())
    }

    pub fn run_prompt(&mut self) -> io::Result<()> {
        let input = io::stdin();
        let mut reader = input.lock();

        println!("\n");
        println!("--------------");
        println!("Welcome to Lox");
        println!("--------------");
        println!("\n");

        loop {
            print!("> ");

            io::stdout().flush().unwrap();

            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(bytes_read) if bytes_read > 0 => {
                    self.run(line);
                }
                Err(error) => {
                    self.error_reporter.borrow_mut().set_error(false);
                    self.error_reporter
                        .borrow_mut()
                        .report_error_message(0, &error.to_string());
                    break;
                }
                Ok(_) => break, // EOF (Ctrl+D on Unix, Ctrl+Z on Windows)
            }
        }

        Ok(())
    }

    pub fn run_prompt_multiline(&mut self) -> io::Result<()> {
        println!("\n\n---------------------------------------");
        println!("--- Welcome to Lox (multiline mode) ---");
        println!("---------------------------------------");
        println!("Ctrl-D to finish input and run the code \n\n");

        let mut lines = String::new();
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            let line = line?;
            if line.trim() == "RUN" {
                println!("\n\nRunning code... \n\n");
                break;
            }
            lines.push_str(&line);
            lines.push('\n');
        }

        self.run(lines.to_string());

        Ok(())
    }

    fn run(&self, source: String) {
        let mut scanner = Scanner::new(source, Rc::clone(&self.error_reporter));
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);

        let mut statements = parser.parse();

        if let Ok(stmts) = &mut statements {
            if let Err(error) = self.interpreter.borrow_mut().interpret(stmts) {
                self.error(LoxError::RuntimeError(error));
            }
        } else if let Err(error) = statements {
            self.error(LoxError::ParseError(error));
        }
    }
}
