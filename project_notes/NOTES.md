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

- token type - keywords, literals, one or two character tokens, single character tokens, ect
- literal value - numbers, strings, ect
- location information
-

_lexical grammar_ - the rules that translate the groups of text characters (lexemes) into tokens (identifiers, keywords, literals, ect)

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
