use crate::{
    ast_grammar::expr::{Expr, ExprVisitor},
    ast_grammar::token::{Literal, Token},
};

pub struct AstPrinter;
impl AstPrinter {
    pub fn _print(expr: Expr) -> String {
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
    fn visit_literal_expr(&self, value: &Option<Literal>) -> String {
        match value {
            Some(Literal::Str(string)) => string.clone(),
            Some(Literal::Num(number)) => number.to_string(),
            _ => String::from("nil"),
        }
    }
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
        self.parenthisize(operator.lexeme.clone(), vec![right])
    }

    fn visit_variable_expr(&self, name: &Token) -> String {
        name.lexeme.clone()
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> String {
        // TODO: Implement this
        String::new()
    }
}
