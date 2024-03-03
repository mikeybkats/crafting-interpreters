use crate::{
    grammar::expr::{Expr, ExprVisitor},
    grammar::token::{Literal, Token},
};

#[derive(Debug)]
pub struct RPNPrinter;
impl RPNPrinter {
    pub fn _print(expr: Expr) -> String {
        return expr.accept(&mut Self);
    }

    fn reverse_notation(&mut self, operator: String, exprs: Vec<&Expr>) -> String {
        let expr_strings: Vec<String> = exprs.iter().map(|&expr| expr.accept(self)).collect();

        format!("{} {}", expr_strings.join(" "), operator)
    }
}

// impl ExprVisitor<String> for RPNPrinter {
//     fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
//         self.reverse_notation(operator.lexeme.clone(), vec![left, right])
//     }
//     fn visit_grouping_expr(&mut self, expression: &Expr) -> String {
//         self.reverse_notation("group".to_string(), vec![expression])
//     }
//     fn visit_literal_expr(&mut self, value: &Option<Literal>) -> String {
//         match value {
//             Some(Literal::Str(string)) => string.clone(),
//             Some(Literal::Num(number)) => number.to_string(),
//             _ => String::from("nil"),
//         }
//     }
//     fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> String {
//         self.reverse_notation(operator.lexeme.clone(), vec![right])
//     }

//     fn visit_variable_expr(&mut self, _name: &Token) -> String {
//         // TODO: Implement this
//         String::new()
//     }

//     fn visit_assign_expr(&mut self, _name: &Token, _value: &Expr) -> String {
//         // TODO: Implement this
//         String::new()
//     }
// }
