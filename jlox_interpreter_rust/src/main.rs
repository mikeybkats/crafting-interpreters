use crate::lox::Lox;

mod error;
mod lox;
mod scanner;
mod token;

fn main() {
    println!("Hello, world!");

    let mut lox = Lox::new();
    lox.main().unwrap();
}
