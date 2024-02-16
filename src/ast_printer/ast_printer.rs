use crate::{
    ast_grammar::expr::{Expr, ExprVisitor},
    ast_grammar::token::{Literal, Token},
};

pub struct AstPrinter;
impl AstPrinter {
    pub fn _print(expr: Expr) -> String {
        return expr.accept(&mut Self);
    }

    fn parenthisize(&mut self, name: String, exprs: Vec<&Expr>) -> String {
        let expr_strings: Vec<String> = exprs.iter().map(|&expr| expr.accept(self)).collect();

        format!("({} {})", name, expr_strings.join(" "))
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthisize(operator.lexeme.clone(), vec![left, right])
    }
    fn visit_grouping_expr(&mut self, expression: &Expr) -> String {
        self.parenthisize("group".to_string(), vec![expression])
    }
    fn visit_literal_expr(&mut self, value: &Option<Literal>) -> String {
        match value {
            Some(Literal::Str(string)) => string.clone(),
            Some(Literal::Num(number)) => number.to_string(),
            _ => String::from("nil"),
        }
    }
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> String {
        self.parenthisize(operator.lexeme.clone(), vec![right])
    }

    fn visit_variable_expr(&mut self, name: &Token) -> String {
        name.lexeme.clone()
    }

    fn visit_assign_expr(&mut self, _name: &Token, _value: &Expr) -> String {
        // TODO: Implement this
        String::new()
    }
}
