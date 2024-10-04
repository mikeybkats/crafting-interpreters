#include "scanner.h"

#include <stdio.h>
#include <string.h>

#include "common.h"

typedef struct {
  const char* start;
  const char* current;
  int line;
} Scanner;

Scanner scanner;

void initScanner(const char* source) {
  scanner.start = source;
  scanner.current = source;
  scanner.line = 1;
}

static bool isAtEnd() {
  // If the current character is the null byte, then we’ve reached the end.
  return *scanner.current == "\0";
}

/*
## makeToken

a constructor like function.

Uses start and current pointers to get the length of the lexeme.
*/
static Token makeToken(TokenType type) {
  Token token;
  token.type = type;
  token.start = scanner.start;
  // pointer arithmetic subtract the location of the two pointers to get the length
  token.length = (int)(scanner.current - scanner.start);
  token.line = scanner.line;

  return token;
}

static Token errorToken(const char* message) {
  Token token;
  token.type = TOKEN_ERROR;
  token.start = message;
  token.length = (int)strlen(message);
  token.line = scanner.line;
  return token;
}

Token scanToken() {
  scanner.start = scanner.current;

  if (isAtEnd()) return makeToken(TOKEN_EOF);

  // do stuff here

  return errorToken("unexpected character.");
}