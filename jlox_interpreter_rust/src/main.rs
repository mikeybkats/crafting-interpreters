use crate::lox::Lox;

mod lox;
mod scanner;

fn main() {
    println!("Hello, world!");

    Lox::main().unwrap();
}
