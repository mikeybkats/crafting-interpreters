# Problem: code has too many nested match statements

```rust
fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Literal {
    let right_literal = self.evaluate(right);

    match right_literal {
        Literal::Num(num) => {
            match operator.token_type {
                TokenType::Minus => Literal::Num(-num),
                _ => Literal::Nil, // Handle other operator cases or invalid operators
            }
        }
        _ => match operator.token_type {
            TokenType::Bang => Literal::Bool(!right_literal.is_truthy()),
            _ => Literal::Nil,
        },
    }
}
```

can you help me simplifiy? and also what are some strategies for making these kinds of matching patterns more simplified?

Nesting `match` statements can indeed become cumbersome, especially when dealing with complex logic. Let's simplify your `visit_unary_expr` function and discuss some strategies to handle nested pattern matching in Rust.

### Simplified `visit_unary_expr` Function

We can simplify your function by combining the `match` statements and minimizing nesting:

```rust
fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Literal {
    let right_literal = self.evaluate(right);

    match (operator.token_type, right_literal) {
        (TokenType::Minus, Literal::Num(num)) => Literal::Num(-num),
        (TokenType::Bang, _) => Literal::Bool(!right_literal.is_truthy()),
        _ => Literal::Nil,
    }
}
```

### Strategies for Simplifying Match Statements

1. **Tuple for Compound Matching**:
   Using a tuple in a `match` statement allows you to match against multiple values simultaneously, as shown above. It's especially useful when your logic depends on the combination of two or more values.

2. **Extracting Common Logic**:
   If you have repeated logic in multiple arms of a `match`, consider extracting it to a separate function or using if-let statements to simplify.

3. **Using `if let` for Single Patterns**:
   If you're only interested in one or two patterns, `if let` can sometimes be more concise than `match`. It's particularly useful when you want to match one pattern and ignore all others.

4. **Guard Clauses in Match Arms**:
   You can use guard clauses in `match` arms to add additional conditions, which can sometimes reduce the need for nested matches.

5. **Early Returns for Error Handling**:
   If some `match` arms represent error conditions, you can sometimes simplify the logic by returning early in those cases.

6. **Refactoring with Helper Functions**:
   If a `match` statement becomes too complex, consider breaking it down into smaller helper functions. Each function can handle a part of the logic, making the overall code more readable.

By applying these strategies, you can often reduce the complexity of nested `match` statements and make your code more readable and maintainable.

## Underscore in pattern matching

The underscore \_ in Rust's pattern matching is a special pattern that acts as a "catch-all" or "ignore" placeholder. When used in a pattern, it means that you're not interested in the value at that position, and you don't intend to use it. Here's what's important about the underscore in pattern matching:

_Ignoring Values_

The underscore \_ is used to ignore values in a pattern match. It's a way of saying, "A value exists here, but I don't care what it is."

_Not a Variable_

Unlike other patterns that bind the matched value to a variable, the underscore does not bind to any value. Therefore, it does not create or represent a variable, and you cannot use it to refer to the matched value later in your code.

_Avoiding Unused Variable Warnings_

Using \_ is also helpful for avoiding compiler warnings about unused variables. If you used a named variable in the pattern and didn't use it in the match arm, Rust's compiler would typically warn you about the unused variable.

_Readability_

It can improve the readability of the code by clearly indicating which parts of a pattern you're actually interested in.
