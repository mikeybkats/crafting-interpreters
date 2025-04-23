#include "compiler.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "chunk.h"
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
 * "These are all of Lox's precedence levels in order from lowest to highest."
 */
typedef enum
{
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

/**
 * ## type ParseFn
 *
 * @brief function pointer to the prefix rule (grouping, number, unary, ect)it
 * can return any function that's void and takes no parameters
 */
typedef void (*ParseFn)(bool canAssign);

typedef struct
{
  ParseFn prefix;         // function pointer to the prefix rule (grouping, number,
                          // unary, ect)
  ParseFn    infix;       // function pointer to the infix rule (binary, etc)
  Precedence precedence;  // Enum value of the precedence of the operator
} ParseRule;

typedef struct
{
  Token name;
  int   depth;
  bool  isConst;
  bool  initialized;
} Local;

typedef struct
{
  Local locals[UINT8_COUNT];
  int   localCount;
  int   scopeDepth;

  Token *initializedGlobals[UINT8_COUNT];
  int    globalsCount;
  bool   isCurrentGlobalConst;
} Compiler;  // added in chapter 22. Compiler not needed until local variables
             // are introduced

Parser parser;  // create a single global variable so state does not need to be
                // passed around
Compiler *current = NULL;
Chunk    *compilingChunk;

#ifdef DEBUG_TEST
extern Chunk *currentChunk() {
  return compilingChunk;
}
#else
static Chunk *currentChunk() {
  return compilingChunk;
}
#endif

static void errorAt(Token *token, const char *message) {
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

static void error(const char *message) {
  errorAt(&parser.previous, message);
}

static void errorAtCurrent(const char *message) {
  errorAt(&parser.current, message);
}

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

static void consume(TokenType type, const char *message) {
  if (parser.current.type == type) {
    advance();
    return;
  }

  errorAtCurrent(message);
}

/**
 * ## check
 *
 * @brief returns true if the current token has the given type
 * @param type the type to check
 */
static bool check(TokenType type) {
  return parser.current.type == type;
}

/**
 * ## match
 *
 * @brief if the current token is the given type, it is consumed and true is
 * returned. Otherwise false is returned.
 */
static bool match(TokenType type) {
  if (!check(type)) return false;
  advance();
  return true;
}

/*
 * ## emitByte
 *
 * @brief emits a byte to the current chunk. The byte is the location of the
 * constant in the values array.
 */
static void emitByte(uint8_t byte) {
  writeChunk(currentChunk(), byte, parser.previous.line);
}

static void emitBytes(uint8_t byte1, uint8_t byte2) {
  emitByte(byte1);
  emitByte(byte2);
}

static void emitReturn() {
  emitByte(OP_RETURN);
}

/*
 * ## makeConstant
 *
 * @brief Adds a constant value to the values array and to the chunk. Checks to
 * see if there are too many constants (256) in one chunk
 */
static uint8_t makeConstant(Value value) {
  // int constant - index of the constant in the values array
  // constant gets added to the currentChunk Values array
  int constant = addConstant(currentChunk(), value);

  if (constant > UINT8_MAX) {
    error("Too many constants in one chunk");

    return 0;
  }

  return (uint8_t)constant;
}

static void emitConstant(Value value) {
  emitBytes(OP_CONSTANT, makeConstant(value));
}

static void initCompiler(Compiler *compiler) {
  memset(compiler->initializedGlobals, 0, sizeof(Token *) * UINT8_COUNT);
  // I don't think the globals need to be explicitly set to NULL
  // for (int i = 0; i < UINT8_COUNT; i++) {
  //   compiler->initializedGlobals[i] = NULL;
  // }
  compiler->globalsCount         = 0;
  compiler->isCurrentGlobalConst = false;
  compiler->localCount           = 0;
  compiler->scopeDepth           = 0;
  current                        = compiler;
}

static void       expression();
static void       statement();
static void       declaration();
static ParseRule *getRule(TokenType type);
static void       parsePrecedence(Precedence precedence);

static uint8_t identifierConstant(Token *name) {
  return makeConstant(OBJ_VAL(copyString(name->start, name->length)));
}

static bool identifiersEqual(Token *a, Token *b) {
  if (a->length != b->length) return false;

  return memcmp(a->start, b->start, a->length) == 0;
}

static int resolveLocal(Compiler *compiler, Token *name) {
  for (int i = compiler->localCount - 1; i >= 0; i--) {
    Local *local = &compiler->locals[i];

    if (identifiersEqual(name, &local->name)) {
      if (local->depth == -1) {
        error("Can't read local variable in its own initializer.");
      }

      return i;
    }
  }

  return -1;
}

static void addLocal(Token name, bool isConst) {
  if (current->localCount == UINT8_COUNT) {
    error("Too many local variables in function");
    return;
  }

  Local *local   = &current->locals[current->localCount++];
  local->name    = name;
  local->isConst = isConst;
  local->depth   = -1;
}

static bool globalInitialized(Token *name) {
  printf("DEBUG -- globalInitialized -- TokenType: %d -- count: %d\n", name->type, current->globalsCount);
  int count;
  if (current->globalsCount == 0) {
    count = 0;
  } else {
    count = current->globalsCount - 1;
  }
  for (int i = count; i != 0; i--) {
    if (identifiersEqual(name, current->initializedGlobals[i])) {
      printf("GLOBAL IS INITALIZED\n");
      return true;
    }
  }
  return false;
}

static void initializeGlobalConst(Token *name) {
  current->initializedGlobals[current->globalsCount] = name;
  current->globalsCount++;
}

static void declareVariable(bool isConst) {
  Token *name = &parser.previous;

  if (current->scopeDepth == 0) {
    if (isConst) {
      current->isCurrentGlobalConst = true;
    }
    return;
  }

  for (int i = current->localCount - 1; i >= 0; i--) {
    Local *local = &current->locals[i];

    // if the depth of the local is less than the depth of the current scope
    // then the name already exists.
    if (local->depth != -1 && local->depth < current->scopeDepth) {
      // it's okay for the name to exist if the depth of the local is less than
      // the current scope because shadowing is allowed.
      break;
    }

    if (identifiersEqual(name, &local->name)) {
      error("Already a variable with this name in this scope");
    }
  }

  addLocal(*name, isConst);
}

static void endCompiler() {
  emitReturn();
#ifdef DEBUG_PRINT_CODE
  if (!parser.hadError) {
    disassembleChunk(currentChunk(), "code");
  }
#endif
}

static void beginScope() {
  current->scopeDepth++;
}

static void endScope() {
  current->scopeDepth--;

  // while the depth of the local is greater than the depth of the current scope
  while (current->localCount > 0 && current->locals[current->localCount - 1].depth > current->scopeDepth) {
    emitByte(OP_POP);
    current->localCount--;
  }
}

static void binary(bool canAssign) {
  TokenType  operatorType = parser.previous.type;
  ParseRule *rule         = getRule(operatorType);

  parsePrecedence((Precedence)(rule->precedence + 1));

  switch (operatorType) {
    case TOKEN_BANG_EQUAL:
      emitBytes(OP_EQUAL, OP_NOT);
      break;
    case TOKEN_EQUAL_EQUAL:
      emitByte(OP_EQUAL);
      break;
    case TOKEN_GREATER:
      emitByte(OP_GREATER);
      break;
    case TOKEN_GREATER_EQUAL:
      emitBytes(OP_LESS, OP_NOT);
      break;
    case TOKEN_LESS:
      emitByte(OP_LESS);
      break;
    case TOKEN_LESS_EQUAL:
      emitBytes(OP_GREATER, OP_NOT);
      break;
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

static void literal(bool canAssign) {
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
static void grouping(bool canAssign) {
  expression();
  consume(TOKEN_RIGHT_PAREN, "Expect ')' after expression");
}

static void number(bool canAssign) {
  double value = strtod(parser.previous.start, NULL);  // string to double
  emitConstant(NUMBER_VAL(value));
}

/**
 * ## string
 *
 * @brief handles strings
 */
static void string(bool canAssign) {
  emitConstant(OBJ_VAL(copyString(parser.previous.start + 1, parser.previous.length - 2)));
}

/**
 * function namedVariable
 * @brief after variable() this is the first variable function to get called from the rules table. It resolves the
 * variable and emits the bytecode to the Chunk.
 */
static void namedVariable(Token name, bool canAssign) {
  uint8_t getOp, setOp;
  int     arg = resolveLocal(current, &name);

  // Remember to bounds check for -1! Always bounds check before indexing or you will suffer!
  if (arg != -1) {
    Local *local = &current->locals[arg];

    if (local->isConst == true) {
      // Locals exist in a locals array, so it's easy to pass flags for if they are
      // constants or not. Globals on the other hand are simply written directly to
      // the constant array in the chunk
      if (local->initialized == true) {
        error("Can't reassign to const variable");
      }

      local->initialized = true;
      getOp              = OP_GET_LOCAL;
      emitBytes(getOp, (uint8_t)arg);
      return;
    }
  }

  bool isInit = globalInitialized(&name);
  if (current->isCurrentGlobalConst && isInit) {
    printf("DEBUG -- namedVariable -- token type: %d -- isCurrentGlobalConst: %s -- isInit: %s\n",
           parser.current.type,
           current->isCurrentGlobalConst ? "true" : "false",
           isInit ? "true" : "false");

    // check if the global is a const and has been added to the initialized array
    // if (parser.current.type == TOKEN_EQUAL && current->isCurrentGlobalConst && globalInitialized(&name)) {
    //   error("Can't reassign to const variable");
    // }

    // set the globalInit flag back to false, since now the global has been set and assignment is complete on the
    // compiler side.
    // current->isCurrentGlobalConst = false;
    // getOp = OP_GET_GLOBAL;
    // emitBytes(getOp, (uint8_t)arg);
    // return;
  }

  if (arg != -1) {
    getOp = OP_GET_LOCAL;
    setOp = OP_SET_LOCAL;
  } else {
    Value value   = OBJ_VAL(copyString(name.start, name.length));
    char *cString = AS_CSTRING(value);
    printf("Initializing global: %s \n", cString);
    initializeGlobalConst(&name);
    arg = identifierConstant(&name);

    getOp = OP_GET_GLOBAL;
    setOp = OP_SET_GLOBAL;
  }

  if (canAssign && match(TOKEN_EQUAL)) {
    expression();
    emitBytes(setOp, (uint8_t)arg);
  } else {
    emitBytes(getOp, (uint8_t)arg);
  }
}

/**
 * function variable
 * @brief gets called from the parse rules during assignment (TOKEN_EQUAL).
 */
static void variable(bool canAssign) {
  namedVariable(parser.previous, canAssign);
}

/*
 * ## unary
 *
 * @brief handles the unary operator
 */
static void unary(bool canAssign) {
  TokenType operatorType = parser.previous.type;

  // compile the operand
  // as with grouping, expression is recursively called
  parsePrecedence(PREC_UNARY);

  // emit the operator instruction
  switch (operatorType) {
    case TOKEN_BANG:
      emitByte(OP_NOT);
      break;
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
    [TOKEN_LEFT_PAREN]    = {grouping,   NULL,       PREC_NONE},
    [TOKEN_RIGHT_PAREN]   = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_LEFT_BRACE]    = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_RIGHT_BRACE]   = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_COMMA]         = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_DOT]           = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_MINUS]         = {   unary, binary,       PREC_TERM},
    [TOKEN_PLUS]          = {    NULL, binary,       PREC_TERM},
    [TOKEN_SEMICOLON]     = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_SLASH]         = {    NULL, binary,     PREC_FACTOR},
    [TOKEN_STAR]          = {    NULL, binary,     PREC_FACTOR},
    [TOKEN_BANG]          = {   unary,   NULL,       PREC_NONE},
    [TOKEN_BANG_EQUAL]    = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_EQUAL]         = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_EQUAL_EQUAL]   = {    NULL, binary,   PREC_EQUALITY},
    [TOKEN_GREATER]       = {    NULL, binary, PREC_COMPARISON},
    [TOKEN_GREATER_EQUAL] = {    NULL, binary, PREC_COMPARISON},
    [TOKEN_LESS]          = {    NULL, binary, PREC_COMPARISON},
    [TOKEN_LESS_EQUAL]    = {    NULL, binary, PREC_COMPARISON},
    [TOKEN_IDENTIFIER]    = {variable,   NULL,       PREC_NONE},
    [TOKEN_STRING]        = {  string,   NULL,       PREC_NONE},
    [TOKEN_NUMBER]        = {  number,   NULL,       PREC_NONE},
    [TOKEN_AND]           = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_CLASS]         = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_ELSE]          = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_FALSE]         = { literal,   NULL,       PREC_NONE},
    [TOKEN_FOR]           = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_FUN]           = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_IF]            = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_NIL]           = { literal,   NULL,       PREC_NONE},
    [TOKEN_OR]            = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_PRINT]         = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_RETURN]        = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_SUPER]         = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_THIS]          = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_TRUE]          = { literal,   NULL,       PREC_NONE},
    [TOKEN_VAR]           = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_CONST]         = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_WHILE]         = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_ERROR]         = {    NULL,   NULL,       PREC_NONE},
    [TOKEN_EOF]           = {    NULL,   NULL,       PREC_NONE},
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
  // prefixRule();
  bool canAssign = precedence <= PREC_ASSIGNMENT;
  prefixRule(canAssign);

  // the infix parsing loop
  // while precedence is less than the current rule's precedence
  while (precedence <= getRule(parser.current.type)->precedence) {
    // advance to the next token
    advance();
    // get the infix rule (because we are parsing the right side of the
    // operator)
    ParseFn infixRule = getRule(parser.previous.type)->infix;
    // else process the rule by calling the infixRule function
    infixRule(canAssign);
  }

  if (canAssign && match(TOKEN_EQUAL)) {
    error("Invalid assignment target.");
  }
}

static uint8_t parseVariable(const char *errorMessage, bool isConst) {
  printf("DEBUG -- parseVariable -- isConst: %s\n", isConst ? "true" : "false");
  consume(TOKEN_IDENTIFIER, errorMessage);

  declareVariable(isConst);
  if (current->scopeDepth > 0) return 0;

  // Set the flag for global const
  if (isConst) {
    current->isCurrentGlobalConst = true;
  }
  return identifierConstant(&parser.previous);
}

/**
 * ## function: markInitialized
 *
 * @brief sets the depth of the top most local variable to the current scope
 * depth.
 *
 * So this is really what "declaring" and "defining" a variable means in the
 * compiler. "Declaring" is when the variable is added to the scope, and
 * "defining" is when it becomes available for use.
 */
static void markInitialized() {
  current->locals[current->localCount - 1].depth = current->scopeDepth;
}

/**
 * ## defineVariable
 *
 * Global variables are looked up by name at runtime. The name is too big to fit
 * into the bytecode. So a string is stored in a constant table and then the
 * instruction refers to the name by index in the table.
 *
 * @brief Outputs the bytecode that defines the new variable and stores its
 * initial value. CH 21.2
 */
static void defineVariable(uint8_t global) {
  // if local variable mark initialized
  if (current->scopeDepth > 0) {
    markInitialized();
    return;
  }

  emitBytes(OP_DEFINE_GLOBAL, global);
}

static ParseRule *getRule(TokenType type) {
  return &rules[type];
}

/**
 * Function: expression
 *
 * @brief  Called when processing an EQUAL token
 */
static void expression() {
  parsePrecedence(PREC_ASSIGNMENT);
}

static void block() {
  while (!check(TOKEN_RIGHT_BRACE) && !check(TOKEN_EOF)) {
    declaration();
  }

  consume(TOKEN_RIGHT_BRACE, "Expect '}' after block.");
}

static void varDeclaration(bool isConst) {
  uint8_t global = parseVariable("Expect variable name.", isConst);

  if (match(TOKEN_EQUAL)) {
    expression();
  } else {
    emitByte(OP_NIL);
  }

  consume(TOKEN_SEMICOLON, "Expect ';' after variable declaration.");

  defineVariable(global);
}

static void expressionStatement() {
  expression();
  consume(TOKEN_SEMICOLON, "Expect ';' after expression.");
  emitByte(OP_POP);
}

static void printStatement() {
  expression();
  consume(TOKEN_SEMICOLON, "Expect ';' after value.");
  emitByte(OP_PRINT);
}

static void synchronize() {
  parser.panicMode = false;

  while (parser.current.type != TOKEN_EOF) {
    if (parser.previous.type == TOKEN_SEMICOLON) return;

    switch (parser.current.type) {
      case TOKEN_CLASS:
      case TOKEN_FUN:
      case TOKEN_VAR:
      case TOKEN_CONST:
      case TOKEN_FOR:
      case TOKEN_IF:
      case TOKEN_WHILE:
      case TOKEN_PRINT:
      case TOKEN_RETURN:
        return;
      default:
        break;
    }

    advance();
  }
}

static void declaration() {
  if (match(TOKEN_CONST)) {
    varDeclaration(true);
  } else if (match(TOKEN_VAR)) {
    varDeclaration(false);
  } else {
    statement();
  }

  if (parser.panicMode) {
    synchronize();
  }
}

static void statement() {
  if (match(TOKEN_PRINT)) {
    printStatement();
  } else if (match(TOKEN_LEFT_BRACE)) {
    beginScope();
    block();
    endScope();
  } else {
    expressionStatement();
  }
}

bool compile(const char *source, Chunk *chunk) {
  initScanner(source);
  Compiler compiler;
  initCompiler(&compiler);

  // initialize the new module variable before we write any bytecode
  compilingChunk = chunk;

  parser.hadError  = false;
  parser.panicMode = false;

  advance();

  while (!match(TOKEN_EOF)) {
    declaration();
  }

  endCompiler();

  return !parser.hadError;
}