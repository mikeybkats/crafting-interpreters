use std::{env, process};

use ast_grammar::expr::Expr;
use scanner::token::{Literal, Token, TokenType};

use crate::{ast_printer::ast_printer::AstPrinter, lox::Lox, rpn_printer::rpn_printer::RPNPrinter};

mod ast_grammar;
mod ast_printer;
mod interpreter;
mod lox;
mod parser;
mod rpn_printer;
mod scanner;

fn main() {
    test_expr();

    let mut lox = Lox::new();

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script[");
        process::exit(64);
    } else if args.len() == 2 {
        lox.run_file(&args[1]).unwrap();
    } else {
        let prompt = lox.run_prompt();

        match prompt {
            Ok(_value) => (),
            Err(_e) => (),
        }
    }

    // Ok(())
}

fn test_expr() {
    let (left, right) = setup_bin_expr();

    let expr = Expr::Binary {
        left: Box::new(left),
        operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
        right: Box::new(right),
    };

    let result = AstPrinter::print(expr);
    println!("{}", result);

    let (left, right) = setup_bin_expr2();

    let expr = Expr::Binary {
        left: Box::new(left),
        operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
        right: Box::new(right),
    };

    let result = RPNPrinter::print(expr);
    println!("{}", result);
    // (* (- 123) (group 45.67))
    // }
}

fn setup_bin_expr2() -> (Expr, Expr) {
    let literal_exp = Box::new(Expr::Literal {
        value: Some(Literal::Num(1.0)),
    });
    let literal_exp2 = Box::new(Expr::Literal {
        value: Some(Literal::Num(2.0)),
    });
    let binary_exp = Expr::Binary {
        left: literal_exp,
        operator: Token::new(TokenType::Plus, "+".to_string(), None, 1),
        right: literal_exp2,
    };

    let literal_exp3 = Box::new(Expr::Literal {
        value: Some(Literal::Num(4.0)),
    });
    let literal_exp4 = Box::new(Expr::Literal {
        value: Some(Literal::Num(3.0)),
    });
    let binary_exp2 = Expr::Binary {
        left: literal_exp3,
        operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
        right: literal_exp4,
    };

    (binary_exp, binary_exp2)
}

fn setup_bin_expr() -> (Expr, Expr) {
    let unary_operator = Token::new(TokenType::Minus, "-".to_string(), None, 1);
    let unary_right = Box::new(Expr::Literal {
        value: Some(Literal::Num(123.0)),
    });
    let unary_expr = Expr::Unary {
        operator: unary_operator,
        right: unary_right,
    };

    let grouping_expr = Expr::Grouping {
        expression: Box::new(Expr::Literal {
            value: Some(Literal::Num(45.67)),
        }),
    };

    (unary_expr, grouping_expr)
}
