#ifndef clox_scanner_h
#define clox_scanner_h

typedef struct
{
  const char* start;    // pointer to the beginning of the current token
  const char* current;  // pointer to the current character of the source code
  int         line;
} Scanner;

typedef enum
{
  TOKEN_LEFT_PAREN,  // Single-character tokens.
  TOKEN_RIGHT_PAREN,
  TOKEN_LEFT_BRACE,
  TOKEN_RIGHT_BRACE,
  TOKEN_COMMA,
  TOKEN_DOT,
  TOKEN_MINUS,
  TOKEN_PLUS,
  TOKEN_SEMICOLON,
  TOKEN_SLASH,
  TOKEN_STAR,  // One or two character tokens.
  TOKEN_BANG,
  TOKEN_BANG_EQUAL,
  TOKEN_EQUAL,
  TOKEN_EQUAL_EQUAL,
  TOKEN_GREATER,
  TOKEN_GREATER_EQUAL,
  TOKEN_LESS,
  TOKEN_LESS_EQUAL,            // Template literals
  TOKEN_LEFT_STRING_LITERAL,   // two characters
  TOKEN_RIGHT_STRING_LITERAL,  // one character
  TOKEN_IDENTIFIER,            // Literals.
  TOKEN_STRING,
  TOKEN_NUMBER,  // Keywords.
  TOKEN_AND,
  TOKEN_CLASS,
  TOKEN_ELSE,
  TOKEN_FALSE,
  TOKEN_FOR,
  TOKEN_FUN,
  TOKEN_IF,
  TOKEN_NIL,
  TOKEN_OR,
  TOKEN_PRINT,
  TOKEN_RETURN,
  TOKEN_SUPER,
  TOKEN_THIS,
  TOKEN_TRUE,
  TOKEN_VAR,
  TOKEN_CONST,
  TOKEN_WHILE,
  TOKEN_ERROR,
  TOKEN_EOF
} TokenType;

/*
## Token

type - TokenType ;
start - char* ;
length - int;
line - int;

"In clox, tokens only store the lexeme—the character sequence exactly as it appears in the user’s source code."

In jlox the token stored an Object for the runtime value which was converted from the token's lexeme
*/
typedef struct
{
  TokenType   type;
  const char* start;  // pointer to the first character of the current token
  int         length;
  int         line;
} Token;

void initScanner(const char* source);

Token scanToken();

#endif