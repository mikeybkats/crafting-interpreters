# Crafting Interpreters

## Part One: The tree walk interpreter in Rust

JLox becomes Rust Lox. Learning Rust by way of Robert Nystrom's Crafting Interpreter's.

The rust_lox directory covers the first half of the book - the JLox (Java Lox) interpreter.

### running the REPL:

From the rust_lox directory: `cargo run`

### Running the REPL in multiline mode:

From the rust_lox directory: `cargo run -- --multiline`

Typing the word `RUN` followed by `Enter` will submit the code.

## Part Two: The bytcode interpreter

After building the tree walk interpreter in Rust, I decided to change gears and complete the rest of
the story using C. Rust is a fascinating language, one I will definately use in the future. But for
now, in an effort to simply 'complete' the book, the `c_lox` directory will follow along with the
\_Crafting Interpreters\* book (including challenges).
