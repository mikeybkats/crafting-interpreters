use std::env;
use std::process;

use generate_ast::GenerateAst;

mod generate_ast;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: generate_ast <output directory>");
        process::exit(64);
    }

    let output_dir = &args[1];

    let generate_ast = GenerateAst::new();

    generate_ast
        .define_ast(
            output_dir.clone(),
            "Expr".to_string(),
            Vec::from([
                "Binary   : Expr left, Token operator, Expr right",
                "Grouping : Expr expression",
                "Literal  : Object value",
                "Unary    : Token operator, Expr right",
            ]),
        )
        .unwrap();
}
