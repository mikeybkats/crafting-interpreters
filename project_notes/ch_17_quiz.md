# Scanner and Parser Concepts Quiz

## Q1: Scanner Pointers
**Q:** If the source code is "123 + 456" and the scanner has just finished processing "123", where would `scanner.start` and `scanner.current` point to?

**A:** scanner.start would point to the beginning of the code string. scanner.current would point to one after 123 -- the whitespace.

## Q2: Token Creation
**Q:** When `makeToken()` is called, how does it know how many characters belong to the current token?

**A:** It uses the scanner.current and scanner.start and pointer arithmetic. Subtract the start from the current to get the length.

## Q3: Parser State
**Q:** Why does the Parser keep track of both a `current` and `previous` token? What's the purpose of each?

**A:** The previous token is used to make the token current token, the current token is defined after the token is made so that the next iteration of the parser starts with the correct token. This two-token system is especially important for parsing expressions where you need to know:
- `previous`: What you just processed (like a number)
- `current`: What you're about to process (like an operator)

## Q4: Memory Management
**Q:** Do Tokens store their own copy of the source code text? Why or why not?

**A:** No, tokens don't store their own copy of the source code. They store a pointer to it. This way data does not need to be copied when referencing the source code in other functions. This is memory efficient because:
- No string copying
- No memory allocation for token text
- Direct access to source for error reporting

## Q5: Chunks vs Tokens
**Q:** What's the relationship between a Token (like TOKEN_NUMBER) and what gets written to a Chunk?

**A:** A chunk holds the array of the data required to perform the expression. The Token holds the Token Name so that the correct op codes can be used that allow us to process data. For example:

## Q6: Scanner Reset
**Q:** In `scanToken()`, why is it important that `scanner.start = scanner.current` happens at the beginning?

**A:** All the pointer arithmetic that increments to the next character is dependent on this assignment. If it wasn't assigned at the beginning then the scanner would always start from the beginning of the code string and never be able to advance correctly. Additionally:
- It marks the start of a new lexeme
- Ensures proper error reporting (points to correct location)
- Maintains the scanner's forward progress