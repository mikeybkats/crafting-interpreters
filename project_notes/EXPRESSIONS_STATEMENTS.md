# expressions vs statements

An expression appears as a grouping of constants, variables and operators. It may also include labels which represent more complex operations (functions).

- `1 + 2`
- `a * 5`
- `sum(a, b)`

A statement may appear like this:

- assignment: `x = 10`
- expression inside of statement: `var total = sum(a, b)`
- evaluation: `a * (b + c) + 1`
- branching: `if (x) { run() }`
- definition: `function foo(){ return goo * boo; }`
- try catch blocks

## Key Differences:

- _Evaluation_: An expression is evaluated to produce a value, whereas a statement is executed to perform an action.
- _Value_: Expressions always yield a value, while statements may or may not produce a value.
- _Usage_: Expressions are often used within statements to compute values or as part of larger expressions. Statements, on the other hand, are the building blocks of a program's logic and control flow.
- _Composition_: Expressions can be composed of smaller expressions, while statements are typically standalone units of executable code.
