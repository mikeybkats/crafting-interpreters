use crate::{
    ast_grammar::expr::{Expr, ExprVisitor},
    ast_grammar::token::{Literal, Token},
};

#[derive(Debug)]
pub struct RPNPrinter;
impl RPNPrinter {
    pub fn _print(expr: Expr) -> String {
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
    fn visit_literal_expr(&self, value: &Option<Literal>) -> String {
        match value {
            Some(Literal::Str(string)) => string.clone(),
            Some(Literal::Num(number)) => number.to_string(),
            _ => String::from("nil"),
        }
    }
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
        self.reverse_notation(operator.lexeme.clone(), vec![right])
    }

    fn visit_variable_expr(&self, name: &Token) -> String {
        // TODO: UPDATE THIS
        name.lexeme.clone()
    }
}
