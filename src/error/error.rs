use super::parse_error::ParseError;
use super::runtime_error::RuntimeError;

pub struct ErrorReporter {
    had_error: bool,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn set_error(&mut self, had_error: bool) {
        self.had_error = had_error;
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn report_error_message(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_, message);
        self.had_error = true;
    }

    // static void runtimeError(RuntimeError error) {
    //     System.err.println(error.getMessage() +
    //         "\n[line " + error.token.line + "]");
    //     hadRuntimeError = true;
    //   }
    pub fn report_runtime_error(&mut self, error: RuntimeError) {
        let (message, token) = error.get_error();
        eprintln!("Runtime Error - [line {}]: {}", token.line, message);

        self.had_error = true;
    }

    pub fn report_parse_error(&mut self, error: ParseError) {
        let (message, token) = error.get_error();
        eprintln!(
            "Parse Error - [line {}] Error at: {:?} - {}",
            token.line,
            token.lexeme,
            // token.token_type,
            message
        );
        self.had_error = true;
    }
}
