use super::object::Object;
use super::token::Token;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
/// # Expression
/// Enumerates the different types of expressions.
///
/// ## Differences between Rust and Java implementations
/// in the book, the author uses a GenerateAST.java class to generate the AST classes. This is not necessary in Rust. The enum and struct syntax achieves the same result in a more straightforward way.
///
/// ## First appears in Representing Code (Chapter 5)
/// "Expressions are the first syntax tree nodes we see, introduced in the _Representing Code_ chapter. The main Expr class defines the visitor interface used to dispatch against the specific expression types, and contains the other expression subclasses as nested classes."
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Option<Object>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut impl ExprVisitor<R>) -> R {
        match self {
            Expr::Assign { name, value } => visitor.visit_assign_expr(name, value),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call_expr(callee, paren, arguments),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical_expr(left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            Expr::Variable { name } => visitor.visit_variable_expr(&self, name),
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        // Implement structural equality comparison
        match (self, other) {
            (
                Expr::Assign {
                    name: name1,
                    value: value1,
                },
                Expr::Assign {
                    name: name2,
                    value: value2,
                },
            ) => name1.lexeme == name2.lexeme && value1 == value2,
            (
                Expr::Binary {
                    left: left1,
                    operator: operator1,
                    right: right1,
                },
                Expr::Binary {
                    left: left2,
                    operator: operator2,
                    right: right2,
                },
            ) => left1 == left2 && operator1.lexeme == operator2.lexeme && right1 == right2,
            (
                Expr::Call {
                    callee: callee1,
                    paren: paren1,
                    arguments: arguments1,
                },
                Expr::Call {
                    callee: callee2,
                    paren: paren2,
                    arguments: arguments2,
                },
            ) => callee1 == callee2 && paren1.lexeme == paren2.lexeme && arguments1 == arguments2,
            (
                Expr::Grouping {
                    expression: expression1,
                },
                Expr::Grouping {
                    expression: expression2,
                },
            ) => expression1 == expression2,
            (Expr::Literal { value: value1 }, Expr::Literal { value: value2 }) => {
                match (value1, value2) {
                    (Some(obj1), Some(obj2)) => match (obj1, obj2) {
                        (Object::Bool(b1), Object::Bool(b2)) => b1 == b2,
                        (Object::Nil, Object::Nil) => true,
                        (Object::Num(n1), Object::Num(n2)) => n1 == n2,
                        (Object::Str(s1), Object::Str(s2)) => s1 == s2,
                        (Object::Callable(_), Object::Callable(_)) => true,
                        _ => false,
                    },
                    (None, None) => true,
                    _ => false,
                }
            }
            (
                Expr::Logical {
                    left: left1,
                    operator: operator1,
                    right: right1,
                },
                Expr::Logical {
                    left: left2,
                    operator: operator2,
                    right: right2,
                },
            ) => left1 == left2 && operator1.lexeme == operator2.lexeme && right1 == right2,
            (
                Expr::Unary {
                    operator: operator1,
                    right: right1,
                },
                Expr::Unary {
                    operator: operator2,
                    right: right2,
                },
            ) => operator1.lexeme == operator2.lexeme && right1 == right2,
            (Expr::Variable { name: name1 }, Expr::Variable { name: name2 }) => name1 == name2,
            _ => false,
        }
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Expr::Assign { name, value } => {
                name.lexeme.hash(state);
                value.hash(state);
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                left.hash(state);
                operator.lexeme.hash(state);
                right.hash(state);
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                callee.hash(state);
                paren.lexeme.hash(state);
                arguments.hash(state);
            }
            Expr::Grouping { expression } => {
                expression.hash(state);
            }
            Expr::Literal { value } => {
                match value {
                    Some(obj) => match obj {
                        Object::Bool(b) => b.hash(state),
                        Object::Nil => (),
                        Object::Num(n) => {
                            state.write_u8(1); // Variant tag
                                               // Convert f64 to a stable representation for hashing
                                               // Note: This is a simple example; consider edge cases like NaN, Infinity, etc.
                            n.to_bits().hash(state)
                        }
                        Object::Str(s) => s.hash(state),
                        Object::Callable(_) => (),
                    },
                    None => (),
                }
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                left.hash(state);
                operator.lexeme.hash(state);
                right.hash(state);
            }
            Expr::Unary { operator, right } => {
                operator.lexeme.hash(state);
                right.hash(state);
            }
            Expr::Variable { name } => {
                name.hash(state);
            }
        }
    }
}

pub trait ExprVisitor<R> {
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> R;
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Expr>) -> R;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> R;
    fn visit_literal_expr(&mut self, value: &Option<Object>) -> R;
    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> R;
    fn visit_variable_expr(&mut self, expr: &Expr, name: &Token) -> R;
}

#[cfg(test)]
mod tests {
    use crate::{environment::generate_id, grammar::token::TokenType};

    use super::*; // Import everything from the outer module

    #[test]
    fn test_literal_equality() {
        let expr1 = Expr::Literal {
            value: Some(Object::Num(42.0)),
        };
        let expr2 = Expr::Literal {
            value: Some(Object::Num(42.0)),
        };
        assert_eq!(
            expr1, expr2,
            "Literal expressions with the same value should be equal"
        );
    }

    #[test]
    fn test_literal_inequality_different_values() {
        let expr1 = Expr::Literal {
            value: Some(Object::Num(42.0)),
        };
        let expr2 = Expr::Literal {
            value: Some(Object::Num(24.0)),
        };
        assert_ne!(
            expr1, expr2,
            "Literal expressions with different values should not be equal"
        );
    }

    #[test]
    fn test_binary_expression_equality() {
        let left_expr = Box::new(Expr::Literal {
            value: Some(Object::Num(1.0)),
        });
        let right_expr = Box::new(Expr::Literal {
            value: Some(Object::Num(2.0)),
        });
        let operator = Token {
            token_type: TokenType::Plus, // Adjust according to your TokenType definition
            lexeme: "+".to_string(),
            literal: None,
            line: 1,
            _id: generate_id(),
        };

        let expr1 = Expr::Binary {
            left: left_expr.clone(),
            operator: operator.clone(),
            right: right_expr.clone(),
        };
        let expr2 = Expr::Binary {
            left: left_expr,
            operator: operator,
            right: right_expr,
        };

        assert_eq!(
            expr1, expr2,
            "Binary expressions with the same structure should be equal"
        );
    }

    #[test]
    fn test_binary_expression_inequality_different_operators() {
        let left_expr = Box::new(Expr::Literal {
            value: Some(Object::Num(1.0)),
        });
        let right_expr = Box::new(Expr::Literal {
            value: Some(Object::Num(2.0)),
        });
        let operator1 = Token {
            token_type: TokenType::Plus, // Adjust according to your TokenType definition
            lexeme: "+".to_string(),
            literal: None,
            line: 1,
            _id: generate_id(),
        };
        let operator2 = Token {
            token_type: TokenType::Minus, // Adjust according to your TokenType definition
            lexeme: "-".to_string(),
            literal: None,
            line: 1,
            _id: generate_id(),
        };

        let expr1 = Expr::Binary {
            left: left_expr.clone(),
            operator: operator1,
            right: right_expr.clone(),
        };
        let expr2 = Expr::Binary {
            left: left_expr,
            operator: operator2,
            right: right_expr,
        };

        assert_ne!(
            expr1, expr2,
            "Binary expressions with different operators should not be equal"
        );
    }

    #[test]
    fn test_variable_expression_equality() {
        let var_expr_1 = Expr::Variable {
            name: Token {
                token_type: TokenType::String,
                lexeme: "2".to_string(),
                literal: Some(Object::Str("2".to_string())),
                line: 200,
                _id: generate_id(),
            },
        };
        let var_expr_2 = Expr::Variable {
            name: Token {
                token_type: TokenType::String,
                lexeme: "2".to_string(),
                literal: Some(Object::Str("2".to_string())),
                line: 500,
                _id: generate_id(),
            },
        };

        assert_eq!(
            var_expr_1, var_expr_2,
            "Testing equality with var expr 1 and var expr 2"
        );
    }

    #[test]
    fn test_variable_expression_inequality() {
        let var_expr_1 = Expr::Variable {
            name: Token {
                token_type: TokenType::String,
                lexeme: "2".to_string(),
                literal: Some(Object::Str("2".to_string())),
                line: 200,
                _id: generate_id(),
            },
        };
        let var_expr_2 = Expr::Variable {
            name: Token {
                token_type: TokenType::String,
                lexeme: "3".to_string(),
                literal: Some(Object::Str("3".to_string())),
                line: 500,
                _id: generate_id(),
            },
        };

        assert_ne!(
            var_expr_1, var_expr_2,
            "Testing inequality with var expr 1 and var expr 2"
        );
    }

    #[test]
    fn test_grouping_expression_equality() {
        let expr1 = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Some(Object::Num(42.0)),
            }),
        };
        let expr2 = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Some(Object::Num(42.0)),
            }),
        };
        assert_eq!(
            expr1, expr2,
            "Grouping expressions with the same value should be equal"
        );
    }

    #[test]
    fn test_grouping_expression_inequality() {
        let expr1 = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Some(Object::Num(42.0)),
            }),
        };
        let expr2 = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Some(Object::Num(24.0)),
            }),
        };
        assert_ne!(
            expr1, expr2,
            "Grouping expressions with different values should not be equal"
        );
    }

    #[test]
    fn test_unary_expression_equality() {
        let expr1 = Expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
                _id: generate_id(),
            },
            right: Box::new(Expr::Literal {
                value: Some(Object::Num(42.0)),
            }),
        };
        let expr2 = Expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
                _id: generate_id(),
            },
            right: Box::new(Expr::Literal {
                value: Some(Object::Num(42.0)),
            }),
        };
        assert_eq!(
            expr1, expr2,
            "Unary expressions with the same value should be equal"
        );
    }

    #[test]
    fn test_unary_expression_inequality() {
        let expr1 = Expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
                _id: generate_id(),
            },
            right: Box::new(Expr::Literal {
                value: Some(Object::Num(42.0)),
            }),
        };
        let expr2 = Expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
                _id: generate_id(),
            },
            right: Box::new(Expr::Literal {
                value: Some(Object::Num(24.0)),
            }),
        };
        assert_ne!(
            expr1, expr2,
            "Unary expressions with different values should not be equal"
        );
    }

    #[test]
    fn test_logical_expression_equality() {
        let left_expr = Box::new(Expr::Literal {
            value: Some(Object::Bool(true)),
        });
        let right_expr = Box::new(Expr::Literal {
            value: Some(Object::Bool(false)),
        });
        let operator = Token {
            token_type: TokenType::And, // Adjust according to your TokenType definition
            lexeme: "and".to_string(),
            literal: None,
            line: 1,
            _id: generate_id(),
        };

        let expr1 = Expr::Logical {
            left: left_expr.clone(),
            operator: operator.clone(),
            right: right_expr.clone(),
        };
        let expr2 = Expr::Logical {
            left: left_expr,
            operator: operator,
            right: right_expr,
        };

        assert_eq!(
            expr1, expr2,
            "Logical expressions with the same structure should be equal"
        );
    }

    #[test]
    fn test_logical_expression_inequality_different_operators() {
        let left_expr = Box::new(Expr::Literal {
            value: Some(Object::Bool(true)),
        });
        let right_expr = Box::new(Expr::Literal {
            value: Some(Object::Bool(false)),
        });
        let operator1 = Token {
            token_type: TokenType::And, // Adjust according to your TokenType definition
            lexeme: "and".to_string(),
            literal: None,
            line: 1,
            _id: generate_id(),
        };
        let operator2 = Token {
            token_type: TokenType::Or, // Adjust according to your TokenType definition
            lexeme: "or".to_string(),
            literal: None,
            line: 1,
            _id: generate_id(),
        };

        let expr1 = Expr::Logical {
            left: left_expr.clone(),
            operator: operator1,
            right: right_expr.clone(),
        };
        let expr2 = Expr::Logical {
            left: left_expr,
            operator: operator2,
            right: right_expr,
        };

        assert_ne!(
            expr1, expr2,
            "Logical expressions with different operators should not be equal"
        );
    }

    #[test]
    fn test_assign_expression_equality() {
        let expr1 = Expr::Assign {
            name: Token {
                token_type: TokenType::String,
                lexeme: "x".to_string(),
                literal: Some(Object::Str("x".to_string())),
                line: 200,
                _id: generate_id(),
            },
            value: Box::new(Expr::Literal {
                value: Some(Object::Num(42.0)),
            }),
        };
        let expr2 = Expr::Assign {
            name: Token {
                token_type: TokenType::String,
                lexeme: "x".to_string(),
                literal: Some(Object::Str("x".to_string())),
                line: 200,
                _id: generate_id(),
            },
            value: Box::new(Expr::Literal {
                value: Some(Object::Num(42.0)),
            }),
        };

        assert_eq!(
            expr1, expr2,
            "Testing equality with assign expr 1 and assign expr 2"
        );
    }

    #[test]
    fn test_assign_expression_inequality() {
        let expr1 = Expr::Assign {
            name: Token {
                token_type: TokenType::String,
                lexeme: "x".to_string(),
                literal: Some(Object::Str("x".to_string())),
                line: 200,
                _id: generate_id(),
            },
            value: Box::new(Expr::Literal {
                value: Some(Object::Num(42.0)),
            }),
        };
        let expr2 = Expr::Assign {
            name: Token {
                token_type: TokenType::String,
                lexeme: "y".to_string(),
                literal: Some(Object::Str("y".to_string())),
                line: 200,
                _id: generate_id(),
            },
            value: Box::new(Expr::Literal {
                value: Some(Object::Num(42.0)),
            }),
        };

        assert_ne!(
            expr1, expr2,
            "Testing inequality with assign expr 1 and assign expr 2"
        );
    }

    #[test]
    fn test_call_expression_equality() {
        let expr1 = Expr::Call {
            callee: Box::new(Expr::Variable {
                name: Token {
                    token_type: TokenType::String,
                    lexeme: "x".to_string(),
                    literal: Some(Object::Str("x".to_string())),
                    line: 200,
                    _id: generate_id(),
                },
            }),
            paren: Token {
                token_type: TokenType::LeftParen,
                lexeme: "(".to_string(),
                literal: None,
                line: 200,
                _id: generate_id(),
            },
            arguments: vec![Expr::Literal {
                value: Some(Object::Num(42.0)),
            }],
        };
        let expr2 = Expr::Call {
            callee: Box::new(Expr::Variable {
                name: Token {
                    token_type: TokenType::String,
                    lexeme: "x".to_string(),
                    literal: Some(Object::Str("x".to_string())),
                    line: 200,
                    _id: generate_id(),
                },
            }),
            paren: Token {
                token_type: TokenType::LeftParen,
                lexeme: "(".to_string(),
                literal: None,
                line: 200,
                _id: generate_id(),
            },
            arguments: vec![Expr::Literal {
                value: Some(Object::Num(42.0)),
            }],
        };

        assert_eq!(
            expr1, expr2,
            "Testing equality with call expr 1 and call expr 2"
        );
    }

    #[test]
    fn test_call_expression_inequality() {
        let expr1 = Expr::Call {
            callee: Box::new(Expr::Variable {
                name: Token {
                    token_type: TokenType::String,
                    lexeme: "x".to_string(),
                    literal: Some(Object::Str("x".to_string())),
                    line: 200,
                    _id: generate_id(),
                },
            }),
            paren: Token {
                token_type: TokenType::LeftParen,
                lexeme: "(".to_string(),
                literal: None,
                line: 200,
                _id: generate_id(),
            },
            arguments: vec![Expr::Literal {
                value: Some(Object::Num(42.0)),
            }],
        };
        let expr2 = Expr::Call {
            callee: Box::new(Expr::Variable {
                name: Token {
                    token_type: TokenType::String,
                    lexeme: "y".to_string(),
                    literal: Some(Object::Str("y".to_string())),
                    line: 200,
                    _id: generate_id(),
                },
            }),
            paren: Token {
                token_type: TokenType::LeftParen,
                lexeme: "(".to_string(),
                literal: None,
                line: 200,
                _id: generate_id(),
            },
            arguments: vec![Expr::Literal {
                value: Some(Object::Num(42.0)),
            }],
        };

        assert_ne!(
            expr1, expr2,
            "Testing inequality with call expr 1 and call expr 2"
        );
    }
}
