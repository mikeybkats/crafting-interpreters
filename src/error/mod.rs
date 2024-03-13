pub mod error;
pub mod lox_return;
pub mod parse_error;
pub mod runtime_error;

pub enum LoxError {
    ParseError(parse_error::ParseError),
    RuntimeError(runtime_error::RuntimeError),
    LoxReturn(lox_return::LoxReturn),
}
