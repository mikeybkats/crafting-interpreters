use std::{
    cell::RefCell,
    fs,
    io::{self, BufRead, Write},
    process,
    rc::Rc,
};

use crate::{
    interpreter::{interpreter::Interpreter, runtime_error::RuntimeError},
    parser::{parse_error::ParseError, parser::Parser},
    scanner::scanner::Scanner,
    scanner::{error::ErrorReporter, token::Literal},
};

pub enum LoxError {
    RuntimeError(RuntimeError),
    ParseError(ParseError),
}

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

        println!("Welcome to Lox");
        println!("--------------");

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

    fn run(&self, source: String) {
        let mut scanner = Scanner::new(source, Rc::clone(&self.error_reporter));
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);

        let statements = parser.parse();

        match statements {
            Ok(stmts) => {
                let result = Interpreter.interpret(stmts);
                match result {
                    Ok(literal) => match literal {
                        Literal::Str(s) => println!("{}", s),
                        Literal::Num(n) => println!("{}", n),
                        Literal::Bool(b) => println!("{}", b),
                        Literal::Nil => (),
                    },
                    Err(error) => self.error(LoxError::RuntimeError(error)),
                }
            }
            Err(error) => self.error(LoxError::ParseError(error)),
        }
    }
}
