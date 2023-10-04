use crate::token::{StringOrNumber, Token};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Option<StringOrNumber>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn accept<R>(&self, visitor: &impl ExprVisitor<R>) -> R {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}

pub trait ExprVisitor<R> {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_grouping_expr(&self, expression: &Expr) -> R;
    fn visit_literal_expr(&self, value: &Option<StringOrNumber>) -> R;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> R;
}

struct AstPrinter;
impl AstPrinter {
    fn print(expr: Expr) -> String {
        return expr.accept(&Self);
    }

    fn parenthisize(&self, name: String, exprs: Vec<&Expr>) -> String {
        let expr_strings: Vec<String> = exprs.iter().map(|&expr| expr.accept(self)).collect();

        format!("({} {})", name, expr_strings.join(" "))
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthisize(operator.lexeme.clone(), vec![left, right])
    }
    fn visit_grouping_expr(&self, expression: &Expr) -> String {
        self.parenthisize("group".to_string(), vec![expression])
    }
    fn visit_literal_expr(&self, value: &Option<StringOrNumber>) -> String {
        match value {
            Some(value) => match value {
                StringOrNumber::Str(string) => string.clone(),
                _ => String::from("nil"),
            },
            _ => String::from("nil"),
        }
    }
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
        self.parenthisize(operator.lexeme.clone(), vec![right])
    }
}

// example usage
// struct MyVisitor;
// impl ExprVisitor<()> for MyVisitor {
//     fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) {
//         println!("lol wat, this is so awesome!");
//     }
//     fn visit_grouping_expr(&self, expression: &Expr) -> () {}
//     fn visit_literal_expr(&self, value: &Option<StringOrNumber>) -> () {}
//     fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> () {}
// }

// fn my_visitor_usage() {
//     let expr = Expr::Binary {
//         left: Box::new(Expr::Literal {
//             value: Some(StringOrNumber::Num(1.0)),
//         }),
//         operator: Token::new(
//             crate::token::TokenType::And,
//             "some string".to_string(),
//             Some(StringOrNumber::Str("()".to_string())),
//             0,
//         ),
//         right: Box::new(Expr::Literal {
//             value: Some(StringOrNumber::Num(9.0)),
//         }),
//     };
//     let mut my_visitor = MyVisitor;
//     expr.accept(&mut my_visitor);
// }
