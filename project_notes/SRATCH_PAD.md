````c
typedef enum
{
  PREC_NONE,        // 0
  PREC_ASSIGNMENT,  // 1:  =
  PREC_OR,          // 2: or
  PREC_AND,         // 3: and
  PREC_EQUALITY,    // 4: == !=
  PREC_COMPARISON,  // 5: < > <= >=
  PREC_TERM,        // 6: + -
  PREC_FACTOR,      // 7: * /
  PREC_UNARY,       // 8: ! -
  PREC_CALL,        // 9: . ()
  PREC_PRIMARY      // 10:
} Precedence;


```c
typedef enum
{
    ...
    TOKEN_PLUS,
    TOKEN_STAR,
    TOKEN_IDENTIFIER,            // Literals.
    TOKEN_NUMBER,  // Keywords.
    TOKEN_VAR,
    ...
} TokenType
````

```
a + b \* c
```

calls to `parsePrecedence(precedence)` are made when processing an expression:

| step | action                                                                                | parser.previous | parser.current         | precedence                         | prefixRule | infixRule | stack effect                              |
| ---- | ------------------------------------------------------------------------------------- | --------------- | ---------------------- | ---------------------------------- | ---------- | --------- | ----------------------------------------- |
| 1    | `// begin parsePrecedence`                                                            | -               | `a` `TOKEN_IDENTIFIER` | `PREC_ASSIGNMENT`                  | -          | -         | -                                         |
| 2    | `advance()`                                                                           | `a`             | `+` `TOKEN_PLUS`       | `PREC_ASSIGNMENT`                  | -          | -         | -                                         |
| 3    | `prefixRule = getRule(TOKEN_IDENTIFIER)->prefix` // variable                          | `a`             | `+`                    | `PREC_ASSIGNMENT`                  | `variable` | -         | -                                         |
| 4    | `variable()`                                                                          | -               | -                      | -                                  | -          | -         | pushes `a` to stack                       |
| 5    | `precedence <= current.precedence // PREC_ASSIGNMENT <= PREC_TERM // true`            | `a`             | `+`                    | `PREC_ASSIGNMENT`                  | -          | -         | -                                         |
| 6    | `advance()`                                                                           | `+`             | `b` `TOKEN_IDENTIFIER` | `PREC_ASSIGNMENT`                  | -          | -         | -                                         |
| 7    | `infixRule = getRule(TOKEN_PLUS)->infix` // binary                                    | `+`             | `b`                    | `PREC_ASSIGNMENT`                  | -          | `binary`  | -                                         |
| 8    | `binary()`                                                                            |                 |                        |                                    | -          | -         |                                           |
| 9    | `parsePrecedence(PREC_FACTOR)`                                                        |                 |                        | (`PREC_TERM` + 1) // `PREC_FACTOR` | -          | -         | -                                         |
| 10   | `advance()`                                                                           | `b`             | `*` `TOKEN_STAR`       | `PREC_FACTOR`                      | -          | -         | -                                         |
| 11   | `prefixRule = getRule(TOKEN_IDENTIFIER)->prefix` // variable                          | `b`             | `*`                    | `PREC_FACTOR`                      | `variable` | -         | -                                         |
| 12   | `variable()`                                                                          | -               | -                      | -                                  | -          | -         | pushes `b` to stack                       |
| 13   | `precedence <= current.precedence // PREC_FACTOR <= PREC_FACTOR // true               | `b`             | `*`                    | `PREC_FACTOR`                      |            |           |                                           |
| 14   | `advance()                                                                            | `*`             | `c` `TOKEN_IDENTIFIER` | `PREC_FACTOR`                      |            |           |                                           |
| 15   | `infixRule = getRule(TOKEN_STAR)->infix` // binary                                    | `*`             | `c`                    |                                    |            |           |                                           |
| 16   | `binary()`                                                                            |                 |                        |                                    |            |           |                                           |
| 17   | `parsePrecedence(PREC_UNARY)`                                                         |                 |                        | (`PREC_FACTOR` + 1) //             |            |           |                                           |
| 18   | `advance()`                                                                           | `c`             | `EOF`                  | `PREC_UNARY`                       |            |           |                                           |
| 19   | `prefixRule = getRule(TOKEN_IDENTIFIER)->prefix` // variable                          | `a`             | `EOF`                  | `PREC_UNARY`                       |            |           |                                           |
| 20   | `variable()`                                                                          | -               | -                      |                                    |            |           |                                           |
| 21   | `precedence <= current.precedence // PREC_UNARY <= PREC_TERM // false`                |                 |                        |                                    |            |           | pushes `c` to stack                       |
| 22   | exits `parsePrecedence` for `PREC_UNARY` returns from `parsePrecedence` in `binary()` |                 |                        |                                    |            |           |                                           |
| 23   | complete `*` - stack effect in `binary()`                                             |                 |                        |                                    |            |           | pop `c`, pop `b`, push `b` \* `c`         |
| 24   | exits `parsePrecedence` for `PREC_FACTOR` returns from `parsePrecedence` in binary()  |                 |                        |                                    |            |           |                                           |
| 25   | complete `+` - stack effect in `binary()`                                             |                 |                        |                                    |            |           | pop `a`, pop `c*b`, push `a` \+ `(b * c)` |
