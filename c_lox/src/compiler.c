#include "compiler.h"

#include <stdio.h>
#include <stdlib.h>

#include "common.h"
#include "scanner.h"

#ifdef DEBUG_PRINT_CODE
#include "debug.h"
#endif

// typedef struct {
//   Token current;
//   Token previous;
//   bool hadError;
//   bool panicMode;
// } Parser;

/*
 * ## Enum: Precedence
 *
 * @brief The precedence of operators
 *
 * "These are all of Lox’s precedence levels in order from lowest to highest."
 */
typedef enum {
  PREC_NONE,
  PREC_ASSIGNMENT,  // =
  PREC_OR,          // or
  PREC_AND,         // and
  PREC_EQUALITY,    // == !=
  PREC_COMPARISON,  // < > <= >=
  PREC_TERM,        // + -
  PREC_FACTOR,      // * /
  PREC_UNARY,       // ! -
  PREC_CALL,        // . ()
  PREC_PRIMARY
} Precedence;

typedef void (*ParseFn)();

typedef struct {
  ParseFn prefix;         // function pointer to the prefix rule (grouping, number, unary, ect)
  ParseFn infix;          // function pointer to the infix rule (binary, etc)
  Precedence precedence;  // Enum value of the precedence of the operator
} ParseRule;

Parser parser;  // create a single global variable so state does not need to be
                // passed around
Chunk* compilingChunk;

#ifdef DEBUG_TEST
extern Chunk* currentChunk() { return compilingChunk; }
#else
static Chunk* currentChunk() { return compilingChunk; }
#endif

static void errorAt(Token* token, const char* message) {
  if (parser.panicMode) return;
  parser.panicMode = true;

  fprintf(stderr, "[line %d] Error", token->line);

  if (token->type == TOKEN_EOF) {
    fprintf(stderr, " at end");
  } else if (token->type == TOKEN_ERROR) {
    // Nothing.
  } else {
    fprintf(stderr, " at '%.*s'", token->length, token->start);
  }

  fprintf(stderr, ": %s\n", message);
  parser.hadError = true;
}

static void error(const char* message) { errorAt(&parser.previous, message); }

static void errorAtCurrent(const char* message) { errorAt(&parser.current, message); }

static void advance() {
  parser.previous = parser.current;

  for (;;) {
    parser.current = scanToken();

    if (parser.current.type != TOKEN_ERROR)
      break;  // Error tokens are created by the scanner, but the parser itself
              // does the error reporting
    errorAtCurrent(parser.current.start);
  }
}

#ifdef DEBUG_TEST
void test_advance() {
  parser.hadError  = false;
  parser.panicMode = false;
  advance();
}
#endif

static void consume(TokenType type, const char* message) {
  if (parser.current.type == type) {
    advance();
    return;
  }

  errorAtCurrent(message);
}

/*
 * ## emitByte
 *
 * @brief emits a byte to the current chunk. The byte is the location of the constant in the values array.
 */
static void emitByte(uint8_t byte) { writeChunk(currentChunk(), byte, parser.previous.line); }

static void emitBytes(uint8_t byte1, uint8_t byte2) {
  emitByte(byte1);
  emitByte(byte2);
}

static void emitReturn() { emitByte(OP_RETURN); }

/*
 * ## makeConstant
 *
 * @brief Adds a constant value to the values array and to the chunk. Checks to see if there are too many constants
 * (256) in one chunk
 */
static uint8_t makeConstant(Value value) {
  // int constant - index of the constant in the values array
  int constant = addConstant(currentChunk(), value);

  if (constant > UINT8_MAX) {
    error("Too many constants in one chunk");

    return 0;
  }

  return (uint8_t)constant;
}

static void emitConstant(Value value) { emitBytes(OP_CONSTANT, makeConstant(value)); }

static void endCompiler() {
  emitReturn();
#ifdef DEBUG_PRINT_CODE
  if (!parser.hadError) {
    disassembleChunk(currentChunk(), "code");
  }
#endif
}

static void expression();
static ParseRule* getRule(TokenType type);
static void parsePrecedence(Precedence precedence);

static void binary() {
  TokenType operatorType = parser.previous.type;
  ParseRule* rule        = getRule(operatorType);

  parsePrecedence((Precedence)(rule->precedence + 1));

  switch (operatorType) {
    case TOKEN_PLUS:
      emitByte(OP_ADD);
      break;
    case TOKEN_MINUS:
      emitByte(OP_SUBTRACT);
      break;
    case TOKEN_STAR:
      emitByte(OP_MULTIPLY);
      break;
    case TOKEN_SLASH:
      emitByte(OP_DIVIDE);
      break;
    default:
      return;  // Unreachable.
  }
}

static void literal() {
  switch (parser.previous.type) {
    case TOKEN_FALSE:
      emitByte(OP_FALSE);
      break;
    case TOKEN_NIL:
      emitByte(OP_NIL);
      break;
    case TOKEN_TRUE:
      emitByte(OP_TRUE);
      break;
    default:
      return;  // Unreachable
  }
}

/*
 * ## grouping
 *
 * @brief handles the grouping operator
 *
 * "as far as the back end is concerned, there is nothing to a grouping
 * expression. It just lets you insert a lower-precedence expression where a
 * higher precedence one is expected."
 */
static void grouping() {
  expression();
  consume(TOKEN_RIGHT_PAREN, "Expect ')' after expression");
}

static void number() {
  double value = strtod(parser.previous.start, NULL);  // string to double
  emitConstant(NUMBER_VAL(value));
}

/*
 * ## unary
 *
 * @brief handles the unary operator
 */
static void unary() {
  TokenType operatorType = parser.previous.type;

  // compile the operand
  // as with grouping, expression is recursively called
  parsePrecedence(PREC_UNARY);

  // emit the operator instruction
  switch (operatorType) {
    case TOKEN_MINUS:
      // write the negation operator to the chunk. This gets done last due to
      // the order of exectution: Evaluate operatnd and leave the value on the
      // stack, then pop that value, negate it and push the result.
      emitByte(OP_NEGATE);
      break;
    default:
      return;  // Unreachable
  }
}

/*
 * ## rules
 *
 * @brief The rules for the parser
 *
 * [RULE_INDEX] = { prefix function, infix function, precedence }
 */
ParseRule rules[] = {
    [TOKEN_LEFT_PAREN]    = {grouping,   NULL,   PREC_NONE},
    [TOKEN_RIGHT_PAREN]   = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_LEFT_BRACE]    = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_RIGHT_BRACE]   = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_COMMA]         = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_DOT]           = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_MINUS]         = {   unary, binary,   PREC_TERM},
    [TOKEN_PLUS]          = {    NULL, binary,   PREC_TERM},
    [TOKEN_SEMICOLON]     = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_SLASH]         = {    NULL, binary, PREC_FACTOR},
    [TOKEN_STAR]          = {    NULL, binary, PREC_FACTOR},
    [TOKEN_BANG]          = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_BANG_EQUAL]    = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_EQUAL]         = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_EQUAL_EQUAL]   = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_GREATER]       = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_GREATER_EQUAL] = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_LESS]          = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_LESS_EQUAL]    = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_IDENTIFIER]    = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_STRING]        = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_NUMBER]        = {  number,   NULL,   PREC_NONE},
    [TOKEN_AND]           = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_CLASS]         = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_ELSE]          = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_FALSE]         = { literal,   NULL,   PREC_NONE},
    [TOKEN_FOR]           = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_FUN]           = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_IF]            = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_NIL]           = { literal,   NULL,   PREC_NONE},
    [TOKEN_OR]            = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_PRINT]         = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_RETURN]        = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_SUPER]         = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_THIS]          = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_TRUE]          = { literal,   NULL,   PREC_NONE},
    [TOKEN_VAR]           = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_WHILE]         = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_ERROR]         = {    NULL,   NULL,   PREC_NONE},
    [TOKEN_EOF]           = {    NULL,   NULL,   PREC_NONE},
};

/*
 * ## parsePrecedence
 *
 * @brief handles the precedence of operators
 *
 * consider this:
 * ```
 * -a.b + c;
 * ```
 *
 * Here the operand to `-` should just be the a.b expression. But if unary is
 * called, it will recursively eat up all of the expression and treat `-` as
 * lower precedence than the `+` which is not how it should work.
 */
static void parsePrecedence(Precedence precedence) {
  // advance to the next token
  advance();

  // get the prefix rule
  // the first will always be a prefix expression
  ParseFn prefixRule = getRule(parser.previous.type)->prefix;

  // if no rule exists throw an error and return
  if (prefixRule == NULL) {
    error("Expect expression");
    return;
  }
  // else process the rule by calling the prefixRule function
  prefixRule();

  // while precedence is less than the current rule's precedence
  while (precedence <= getRule(parser.current.type)->precedence) {
    printf("DEBUG_TEST - parsePrecedence - precedence: %d - current token: %d\n", precedence, parser.current.type);
    // advance to the next token
    advance();
    // get the infix rule (because we are parsing the right side of the operator)
    ParseFn infixRule = getRule(parser.previous.type)->infix;
    // else process the rule by calling the infixRule function
    infixRule();
  }
}

static ParseRule* getRule(TokenType type) { return &rules[type]; }

static void expression() { parsePrecedence(PREC_ASSIGNMENT); }

bool compile(const char* source, Chunk* chunk) {
  initScanner(source);

  // initialize the new module variable before we write any bytecode
  compilingChunk = chunk;

  parser.hadError  = false;
  parser.panicMode = false;

  advance();
  expression();
  consume(TOKEN_EOF, "Expect end of expression");

  endCompiler();

  return !parser.hadError;
}