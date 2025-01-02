#include "scanner.h"

#include <stdio.h>
#include <string.h>

#include "common.h"

Scanner scanner;

void initScanner(const char* source) {
  scanner.start   = source;
  scanner.current = source;
  scanner.line    = 1;
}

static bool isAlpha(char c) { return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'; }

static bool isDigit(char c) { return c >= '0' && c <= '9'; }

static bool isAtEnd() {
  // If the current character is the null byte, then we’ve reached the end.
  return *scanner.current == '\0';
}

static char scannerAdvance() {
  scanner.current++;           // increments the pointer forward one block
  return scanner.current[-1];  // returns the character that was just moved passed
}

static char peek() { return *scanner.current; }

static char peekNext() {
  if (isAtEnd()) return '\0';
  return scanner.current[1];
}

static bool match(char expected) {
  if (isAtEnd()) return false;

  // if current char == expected return true
  if (*scanner.current != expected) return false;
  scanner.current++;

  return true;
}

/*
## makeToken

A constructor like function. Returns a token data structure / struct.

Uses start and current pointers to get the length of the lexeme.
*/
static Token makeToken(TokenType type) {
  Token token;
  token.type  = type;
  token.start = scanner.start;
  // pointer arithmetic subtract the location of the two pointers to get the length
  token.length = (int)(scanner.current - scanner.start);
  token.line   = scanner.line;

  return token;
}

static Token errorToken(const char* message) {
  Token token;
  token.type   = TOKEN_ERROR;
  token.start  = message;
  token.length = (int)strlen(message);
  token.line   = scanner.line;
  return token;
}

static void skipWhitespace() {
  for (;;) {
    char c = peek();
    switch (c) {
      case ' ':
      case '\r':
      case '\t':
        scannerAdvance();
        break;
      case '\n':
        scanner.line++;
        scannerAdvance();
        break;
      case '/':
        if (peekNext() == '/') {
          // A comment goes until the end of the line.
          while (peek() != '\n' && !isAtEnd()) scannerAdvance();
        } else {
          return;
        }
        break;
      default:
        return;
    }
  }
}

/*
## checkKeyword

Tests the "rest" of a potential keyword's lexeme - "rest" meaning whatever is left after the first character of the
string.

It looks at the current position of the scanner, gets the characters at that position, and compares the rest with those
characters.
*/
static TokenType checkKeyword(int start, int length, const char* rest, TokenType type) {
  if (scanner.current - scanner.start == start + length && memcmp(scanner.start + start, rest, length) == 0) {
    return type;
  }

  return TOKEN_IDENTIFIER;
}

static TokenType identifierType() {
  switch (scanner.start[0]) {
    case 'a':
      return checkKeyword(1, 2, "nd", TOKEN_AND);
    case 'c':
      return checkKeyword(1, 4, "lass", TOKEN_CLASS);
    case 'e':
      return checkKeyword(1, 3, "lse", TOKEN_ELSE);
    case 'f':
      // add a nested switch statement for all the keywords that begin with 'f'
      if (scanner.current - scanner.start > 1) {
        switch (scanner.start[1]) {
          case 'a':
            return checkKeyword(2, 3, "lse", TOKEN_FALSE);
          case 'o':
            return checkKeyword(2, 1, "r", TOKEN_FOR);
          case 'u':
            return checkKeyword(2, 1, "n", TOKEN_FUN);
        }
      }
      break;
    case 'i':
      return checkKeyword(1, 1, "f", TOKEN_IF);
    case 'n':
      return checkKeyword(1, 2, "il", TOKEN_NIL);
    case 'o':
      return checkKeyword(1, 1, "r", TOKEN_OR);
    case 'p':
      return checkKeyword(1, 4, "rint", TOKEN_PRINT);
    case 'r':
      return checkKeyword(1, 5, "eturn", TOKEN_RETURN);
    case 's':
      return checkKeyword(1, 4, "uper", TOKEN_SUPER);
    case 't':
      if (scanner.current - scanner.start > 1) {
        switch (scanner.start[1]) {
          case 'h':
            return checkKeyword(2, 2, "is", TOKEN_THIS);
          case 'r':
            return checkKeyword(2, 2, "ue", TOKEN_TRUE);
        }
      }
      break;
    case 'v':
      return checkKeyword(1, 2, "ar", TOKEN_VAR);
    case 'w':
      return checkKeyword(1, 4, "hile", TOKEN_WHILE);
  }

  return TOKEN_IDENTIFIER;
}

static Token identifier() {
  while (isAlpha(peek()) || isDigit(peek())) scannerAdvance();
  return makeToken(identifierType());
}

static Token number() {
  while (isDigit(peek())) scannerAdvance();

  // Look for a fractional part.
  if (peek() == '.' && isDigit(peekNext())) {
    // consume the ".".
    scannerAdvance();

    while (isDigit(peek())) scannerAdvance();
  }

  return makeToken(TOKEN_NUMBER);
}

static Token string() {
  while (peek() != '"' && !isAtEnd()) {
    if (peek() == '\n') scanner.line++;
    scannerAdvance();
  }

  if (isAtEnd()) return errorToken("Unterminated string.");

  // the closing quote
  scannerAdvance();
  return makeToken(TOKEN_STRING);
}

Token scanToken() {
  skipWhitespace();
  scanner.start = scanner.current;

  if (isAtEnd()) return makeToken(TOKEN_EOF);

  char c = scannerAdvance();

  if (isAlpha(c)) return identifier();
  if (isDigit(c)) return number();

  switch (c) {
    case '(':
      return makeToken(TOKEN_LEFT_PAREN);
    case ')':
      return makeToken(TOKEN_RIGHT_PAREN);
    case '{':
      return makeToken(TOKEN_LEFT_BRACE);
    case '}':
      return makeToken(TOKEN_RIGHT_BRACE);
    case ';':
      return makeToken(TOKEN_SEMICOLON);
    case ',':
      return makeToken(TOKEN_COMMA);
    case '.':
      return makeToken(TOKEN_DOT);
    case '-':
      return makeToken(TOKEN_MINUS);
    case '+':
      return makeToken(TOKEN_PLUS);
    case '/':
      return makeToken(TOKEN_SLASH);
    case '*':
      return makeToken(TOKEN_STAR);
    case '!':
      return makeToken(match('=') ? TOKEN_BANG_EQUAL : TOKEN_BANG);
    case '=':
      return makeToken(match('=') ? TOKEN_EQUAL_EQUAL : TOKEN_EQUAL);
    case '<':
      return makeToken(match('=') ? TOKEN_LESS_EQUAL : TOKEN_LESS);
    case '>':
      return makeToken(match('=') ? TOKEN_GREATER_EQUAL : TOKEN_GREATER);
    case '"':
      return string();
  }

  return errorToken("unexpected character.");
}

#ifdef DEBUG_TEST
const char* test_get_scanner_current(void) { return scanner.current; }

const char* test_get_scanner_start(void) { return scanner.start; }

int test_get_scanner_line(void) { return scanner.line; }
#endif