pub struct ErrorReporter {
    had_error: bool,
}

impl ErrorReporter {
    fn new() -> Self {
        Self { had_error: false }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_, message);
        self.had_error = true;
    }
}

fn main() {
    let mut reporter = ErrorReporter::new();
    reporter.error(1, "An error occurred");
}
