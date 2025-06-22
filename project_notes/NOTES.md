# Vocabulary

_Chomsky hierarchy_ - four different classes of formal grammars exist that can generate increasingly
complex languages:

```
+---------------------------------+
| recursively enumerable          |
|                                 |
|     +---------------------------+
|     | context sensitive         |
|     |                           |
|     |      +--------------------+
|     |      | context free       |
|     |      |                    |
|     |      |       +------------+
|     |      |       | regular    |
|     |      |       |            |
+-----+------+-------+------------+
```

_context free languages (CFL)_ can be described by _Context Free Grammars (CFG)_

_context free grammar_ - A set of atomic pieces are defined as the alphabet. A finite set of strings
is defined as being the language's grammar.

_context sensitive languages (CSL)_

| terminology    | lexical grammar  | syntactic grammar |
| -------------- | ---------------- | ----------------- |
| alphabet is    | characters       | tokens            |
| string is      | lexeme or tokens | expression        |
| implemented by | scanner          | parser            |

_deterministic finite automata (DFA)_ - there is only one transition to the next state. Given the
state and the input the DFA can determine the next state

_finite automata_ - given a sequence of input symbols there are a finite number of states,
transitions between states and actions that can be taken.

_lexeme_ - blobs of characters in the raw source code from which meaning can be derived. Lexemes can
be categorized as data:

- token type - keywords, Objects, one or two character tokens, single character tokens, ect
- Object value - numbers, strings, ect
- location information

_lexical grammar_ - the rules that translate the groups of text characters (lexemes) into tokens
(identifiers, keywords, Objects, ect)

- lexical grammar can be regular or not regular (not describable through regular expressions)

- Python for instance has indentation based scoping, which is managed by tokens for indentation.

- Haskell is not regular because of features like nested comments.

- Both the Python and Haskel examples illustrate that the lexer keeps track of context.

_nondeterministic finite automaton (NFA)_ - there can be greater than or zero transitions from a
state for a given input. Different states could be moved to at once.

_please excuse my dear aunt sally_ - order of operations: parentheses, exponents, multiplication,
addition, subtraction

_pushdown automata_

_regular language_ - language that can be described through regular expressions

## Rules for grammars

_derivations_ - strings that are derived from the rules of the grammar

_productions_ - rules to produce strings from the grammar

_terminal_ - individual tokens that are the most basic in the grammar

_non terminal_ - tokens made from other tokens. a named reference that refers to another rule in the
grammar

## Syntax Trees

The data structure of the grammar will form a tree.

a _parse tree_ includes all grammar productions as nodes.

_abstract syntax tree_ only include the grammar nodes that are needed

## Chapter 11 - Resolving and Binding

#### Lox - Lexical scope

_Lexical Scope_ - a variable value is determined by its location in the source code and the
determination is made at compile time.

```
var a = "outer";
{
  var a = "inner";
  print a;
}
```

In the example above `print a` will result in `inner`. Easy. Here is the scope rule for Lox:

"A variable usage refers to the prGeceding declaration with the same name in the innermost scope
that encloses the expression where the variable is used."

So in this case, given the rule:

```
var a = "global";
{
  fun showA() {
    print a;
  }

  showA();
  var a = "block";
  showA();
}
```

Both `showA()` functions will print "global". This is lexical scope.

#### Other Languages - Dynamic Scope

_dynamic scope_ - the value of a variable is determined at runtime, and the rules for how a variable
resolves can be far more complex and based on the context of execution.

Javascript for instance hoists variables declared inside a block to the top of the definition
context. This was changed with the introduction of the let keyword.

#### Semantic analysis

Semantic analysis is the process of inspecting the structure of the source code and using known
patterns and context to optimize the compilation. Basically, optimize based on known patterns and
context.

## Chapter 12 - Classes

#### _closure_ - "a closure is a function or reference to a function together with a referencing environment—a structure storing the environment in which the closure was created. A closure allows a function to access variables from the scope in which it was declared, even after that scope has ended." CGPT

```javascript
class Cake {
  taste() {
    var adjective = "delicious";
    print "The " + this.flavor + " cake is " + adjective + "!";
  }
}

var cake = Cake();
cake.flavor = "German chocolate";
cake.taste(); // Prints "The German chocolate cake is delicious!".
```

In the example above the closure of the taste method is the surrounding Cake environment. "taste"
accesses the instance of Cake and exists for the lifetime of the cake variable.

#### _stretch goal: setters and getters_:

There are a couple of ways getters could work in the interpreter. Getters are syntactic sugar for
calling a function that's on the class. In a statement like this a couple of things are going on in
the Parser at compile time and Interpreter at runtime:

```
class Container {
  init(x,y,z){
    this.width = x;
    this.height = y;
    this.depth = z;
  }

  volume {
    return this.width * this.height * this.depth;
  }
}

print this.volume // prints this.width * this.height * this.depth
```

_in the class_ - Volume is parsed as a block statement. But block statements don't inherently have
any way of being accessed so it needs to have a relationship to the word volume. Is volume a
variable? A function? It is technically a function.

_in the call_ - How does it get marked though so that the interpreter knows to treat this.volume
(instead of this.volume()) as a function call? The parser has to mark this.volume as a function
call. How does it do that? Normally it would get treated as an identifier.

Does this warrant new grammar? Or, does it become something handled by within functions? It seems
that the best thing to do is to definately add new grammar, because it's a totally different
composed structure – so it needs a new part of speech. However, a getter is just syntactic sugar for
a function. What is the precedent? For loops don't have their own grammar added to the stmt enum
therefore the rule should really be:

`if the new feature encapsulates the base capabilities it's only syntactic sugar and does not warrant new grammar`

In other words, getters are just like for loops in the sense that they only add a better and simpler
way to do what functions already do.

1. make a handler for the class getter function stmt. create a key in the function struct for kind.
   Function kind can be getter or function.
2. make a handler for the accessor. When a function is accessed as a value (not a call) check to see
   if it's a getter. If it is, then treat it like a call expression and call the getter function.
   This pushes the brunt of the work out of the parser and into the compiler.

## Chapter 14 - bytecode

A refresher on big O notation. n represents the size of the input.

| complexity | meaning      | notes                                                                                                                                                                     |
| ---------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| O(1)       | Constant     | Order of one, or constant time. No matter how large the input the time will be the same.                                                                                  |
| O(n)       | Linear       | Order of n, or linear time. The runtime grows proportional to the input.                                                                                                  |
| O(log n)   | Logarithmic  | Order of log n, or logarithmic time. The problem is divided by a constant with each step                                                                                  |
| O(n log n) | Linearithmic | Order n log n, or linearithmic time. With each step n is divided by a constant and then divided by log n. If n is 5, the runtime is 5 (log 5), or roughly 5 ( 2.3 x 2.3 ) |
| O(n^2)     | Quadratic    | Order to the power of 2, or quadratic time. Runtime that doubles with each additional input element.                                                                      |
| O(n!)      | Factorial    | Order n factorial, or factorial time. Runtime grows factorially with n. So if n is 5, the runtime is 5 x 4 x 3 x 2 x 1 = 120                                              |

| i   | j   | entry | data[j] | occurenceCout |
| --- | --- | ----- | ------- | ------------- |
| 0   | 0   | 123   | 123     | 1             |
| 1   | 1   | 123   | 123     | 2             |
| 2   | 2   | 123   | 123     | 3             |
| 3   | 3   | 123   |         |               |

## Chapter 15 - Virtual Machine

VMs represent a middle ground between pure compilation and pure interpretation.

_READ_BYTE_ macro - Given a numeric opcode (the first byte of an instruction) the correct C code
that implements the instructions semantics must be retrieved. This is called _decoding or
dispatching_.

_decoding and dispatching_ - retrieving and using the correct command for the given opcode.

> "Programming language lore is filled with clever techniques to do bytcode dispatch efficiently.
> Alas, the fastest solutions require either non-standard extensions to C, or handwritten assembly
> code.

#### If you want to learn some of these techniques, look up “direct threaded code”, “jump table”, and “computed goto”.

_[direct threaded code]()_

_[jump table]()_

_[computed goto]()_

## Chapter 18

```
(a != b) == !(a == b) // true
```

| a     | b     | !=    | !(a == b) |
| ----- | ----- | ----- | --------- |
| true  | false | true  | true      |
| true  | true  | false | false     |
| false | false | false | false     |
|       |       |       |           |

```
(a <= b) == !(a > b) // true
```

```
(a >= b) == !(a < b)
```

### Revisiting how it works

- _scanner_ - Scans a line of code. Makes a Token. The token has a pointer to referenced line of
  code.
- _compiler_ - Uses the Token to write to a chunk. Compiler emits byte instructions to the chunk.
- _vm_ - Reads from the chunk and executes the instruction

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

The scanner chops up the string into parts around the variables for the identifiers. This seperates
the concerns of strings and variables into smaller parts.

### Q: _Some languages like C++ used double angle brackets to notate some types `>>`. Like this:_

```
vector<vector<string>> nestedVectors;
```

_The above would produce compile errors because the `>>` was lexed to a single right shift token
instead of two `>` tokens. Later versions of C++ fixed this problem._

_How do other languages handle this problem?_

The real answer is context. Most languages handle this problem through context of where the scanner
is scanning in the code. If the double angled bracket appears in a type definition its handled
differently than if it appears in an expression.

- C++ handled the context in the lexer -- not typically where context is handled.
- Java and C# tokenize the `>>` to handle the context in the parser -- typically where context is
  handled.

### _Many languages define "Contextual Keywords." `await` and `async` are two examples in C#. What are some other contextual keywords in other languages?_

Javascript has `try` and `catch`

How would _contextual keywords_ be implemented, if you needed to do this?

- The parser handles the majority of context related situations. The context should be named, and if
  in a special context, new keywords become available. If in that context a keyword is used
  incorrectly then an error is thrown.

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

_infix expressions_ - expressions that the parser does not know it's position in the expression
until after it has parsed the left operand

_left associative_ : Arithmetic is left associative. This `1 + 2 + 3 + 4` will parse like this
`((1 + 2) + 3) + 4`

_right associative_ : Assignment is right associative. This `a = b = c = d` will parse like this
`a = (b = (c = d))`
