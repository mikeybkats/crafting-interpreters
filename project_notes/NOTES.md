# Vocabulary

_Chomsky hierarchy_ - four different classes of formal grammars exist that can generate increasingly complex languages:

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

_context free grammar_ - A set of atomic pieces are defined as the alphabet. A finite set of strings is defined as being the language's grammar.

_context sensitive languages (CSL)_

| terminology    | lexical grammar  | syntactic grammar |
| -------------- | ---------------- | ----------------- |
| alphabet is    | characters       | tokens            |
| string is      | lexeme or tokens | expression        |
| implemented by | scanner          | parser            |

_deterministic finite automata (DFA)_ - there is only one transition to the next state. Given the state and the input the DFA can determine the next state

_finite automata_ - given a sequence of input symbols there are a finite number of states, transitions between states and actions that can be taken.

_lexeme_ - blobs of characters in the raw source code from which meaning can be derived. Lexemes can be categorized as data:

- token type - keywords, Objects, one or two character tokens, single character tokens, ect
- Object value - numbers, strings, ect
- location information
-

_lexical grammar_ - the rules that translate the groups of text characters (lexemes) into tokens (identifiers, keywords, Objects, ect)

- lexical grammar can be regular or not regular (not describable through regular expressions)

- Python for instance has indentation based scoping, which is managed by tokens for indentation.

- Haskell is not regular because of features like nested comments.

- Both the Python and Haskel examples illustrate that the lexer keeps track of context.

_nondeterministic finite automaton (NFA)_ - there can be greater than or zero transitions from a state for a given input. Different states could be moved to at once.

_please excuse my dear aunt sally_ - order of operations: parentheses, exponents, multiplication, addition, subtraction

_pushdown automata_

_regular language_ - language that can be described through regular expressions

## Rules for grammars

_derivations_ - strings that are derived from the rules of the grammar

_productions_ - rules to produce strings from the grammar

_terminal_ - individual tokens that are the most basic in the grammar

_non terminal_ - tokens made from other tokens. a named reference that refers to another rule in the grammar

## Syntax Trees

The data structure of the grammar will form a tree.

a _parse tree_ includes all grammar productions as nodes.

_abstract syntax tree_ only include the grammar nodes that are needed

## Chapter 11 - Resolving and Binding

#### Lox - Lexical scope

_Lexical Scope_ - a variable value is determined by its location in the source code and the determination is made at compile time.

```
var a = "outer";
{
  var a = "inner";
  print a;
}
```

In the example above `print a` will result in `inner`. Easy. Here is the scope rule for Lox:

"A variable usage refers to the prGeceding declaration with the same name in the innermost scope that encloses the expression where the variable is used."

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

_dynamic scope_ - the value of a variable is determined at runtime, and the rules for how a variable resolves can be far more complex and based on the context of execution.

Javascript for instance hoists variables declared inside a block to the top of the definition context. This was changed with the introduction of the let keyword.

#### Semantic analysis

Semantic analysis is the process of inspecting the structure of the source code and using known patterns and context to optimize the compilation. Basically, optimize based on known patterns and context.

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

In the example above the closure of the taste method is the surrounding Cake environment. "taste" accesses the instance of Cake and exists for the lifetime of the cake variable.
