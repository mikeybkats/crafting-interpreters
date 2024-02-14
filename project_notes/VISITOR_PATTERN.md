## translation of visitor pattern to Rust

Crafting interpreters uses the visitor pattern to call the appropriate method when parsing statements and expressions. Is there a point to doing translating a visitor pattern to rust? Why not just use enums and pattern matching and call it a day?

```Rust
enum Expr {
    // contains all expression types
    ExprName {
        name: String
    }
}

impl Expr {
    // accept takes an expression visitor as the argument. this visitor will contain the side effects and behaviors intented to run when matching against the given expression
    pub fn accept<R>(&self, visitor: ExprVisitor<R>) -> R {
        match self {
            // matched expression calls the visitor function when accept is called
            Expr::ExprName { name } => visitor.visit_expr_name(keys)
            // list other expressions to match for
        }
    }
}

pub trait ExprVisitor<R> {
    fn visit_expr_name(&self, expr_keys) -> R;
    // list other visitors
}

struct SomeStruct;
impl SomeStruct {
    fn execute(expr: Expr){
        return expr.accept(&Self);
    }

    // define more behaviors and effects
}

impl ExprVisitor<SomeType> for SomeStruct {
    fn visit_expr_name(&self){
        // call behaviors and effects
    }
}
```
