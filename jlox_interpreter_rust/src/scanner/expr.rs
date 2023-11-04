use crate::scanner::token::{Literal, Token};

#[derive(Debug)]
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
        value: Literal,
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
    fn visit_literal_expr(&self, value: &Literal) -> R;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> R;
}

pub struct AstPrinter;
impl AstPrinter {
    pub fn print(expr: Expr) -> String {
        return expr.accept(&Self);
    }

    fn parenthisize(&self, name: String, exprs: Vec<&Expr>) -> String {
        let expr_strings: Vec<String> = exprs.iter().map(|&expr| expr.accept(self)).collect();

        format!("({} {})", name, expr_strings.join(" "))
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        println!("{:?} {:?}", left, right);
        self.parenthisize(operator.lexeme.clone(), vec![left, right])
    }
    fn visit_grouping_expr(&self, expression: &Expr) -> String {
        self.parenthisize("group".to_string(), vec![expression])
    }
    fn visit_literal_expr(&self, value: &Literal) -> String {
        match value {
            Literal::Str(string) => string.clone(),
            Literal::Num(number) => number.to_string(),
            _ => String::from("nil"),
        }
    }
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
        self.parenthisize(operator.lexeme.clone(), vec![right])
    }
}

// (* (- 123) (group 45.67))
// -123 group 45.67 *

#[derive(Debug)]
pub struct RPNPrinter;
impl RPNPrinter {
    pub fn print(expr: Expr) -> String {
        return expr.accept(&Self);
    }

    fn reverse_notation(&self, operator: String, exprs: Vec<&Expr>) -> String {
        let expr_strings: Vec<String> = exprs.iter().map(|&expr| expr.accept(self)).collect();

        format!("{} {}", expr_strings.join(" "), operator)
    }
}

impl ExprVisitor<String> for RPNPrinter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.reverse_notation(operator.lexeme.clone(), vec![left, right])
    }
    fn visit_grouping_expr(&self, expression: &Expr) -> String {
        self.reverse_notation("group".to_string(), vec![expression])
    }
    fn visit_literal_expr(&self, value: &Literal) -> String {
        match value {
            Literal::Str(string) => string.clone(),
            Literal::Num(number) => number.to_string(),
            _ => String::from("nil"),
        }
    }
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
        self.reverse_notation(operator.lexeme.clone(), vec![right])
    }
}
