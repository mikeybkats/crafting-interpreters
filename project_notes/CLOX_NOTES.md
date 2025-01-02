# CH16 challenges

### Q: _What sequence of tokens would you emit for the string literal:_ `"${drink} will be ready in ${steep + cool} minutes."` 

first idea wrong:
```
TOKEN_STRING
TOKEN_IDENTIFIER
TOKEN_IDENTIFIER
TOKEN_PLUS
TOKEN_IDENTIFIER
TOKEN_STRING
```

second idea correct:
```
TOKEN_STRING
TOKEN_IDENTIFIER (for 'drink')
TOKEN_STRING
TOKEN_IDENTIFIER (for 'steep')
TOKEN_PLUS
TOKEN_IDENTIFIER (for 'cool')
TOKEN_STRING
```

The scanner chops up the string into parts around the variables for the identifiers. This seperates the concerns of strings and variables into smaller parts.

### Q: _Some languages like C++ used double angle brackets to notate some types `>>`. Like this:_

```
vector<vector<string>> nestedVectors;
```

_The above would produce compile errors because the `>>` was lexed to a single right shift token instead of two `>` tokens. Later versions of C++ fixed this problem._

_How do other languages handle this problem?_

The real answer is context. Most languages handle this problem through context of where the scanner is scanning in the code. If the double angled bracket appears in a type definition its handled differently than if it appears in an expression.

- C++ handled the context in the lexer -- not typically where context is handled.
- Java and C# tokenize the `>>` to handle the context in the parser -- typically where context is handled.

### _Many languages define "Contextual Keywords." `await` and `async` are two examples in C#. What are some other contextual keywords in other languages?_

Javascript has `try` and `catch`

How would _contextual keywords_ be implemented, if you needed to do this? 

- The parser handles the majority of context related situations. The context should be named, and if in a special context, new keywords become available. If in that context a keyword is used incorrectly then an error is thrown. 

implementation areas:

- context handling of keywords
    - contexts in parser: `"FUNCTION" | "BLOCK" | "CLASS" | "GLOBAL"` 

in the global context:
```
scan --> 
set context --> "GLOBAL"
word scanned "async" --> 
match keywords --> MATCH
tokenize --> "async" as KEYWORD
```

inside the function or block context:
```
scan --> 
set context --> "BLOCK"
word scanned "async" --> 
match keywords --> NO_MATCH
tokenize --> "async" as IDENTIFIER
```

# CH17 

_prefix expressions_ - expressions that start with a particular token

_infix expressions_ - expressions that the parser does not know it's position in the expression until after it has parsed the left operand

_left associative_ : Arithmetic is left associative. This `1 + 2 + 3 + 4` will parse like this `((1 + 2) + 3) + 4`

_right associative_ : Assignment is right associative. This `a = b = c = d` will parse like this `a = (b = (c = d))`