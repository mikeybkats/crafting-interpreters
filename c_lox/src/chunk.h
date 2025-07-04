#ifndef clox_chunk_h
#define clox_chunk_h

#include "common.h"
#include "memory.h"
#include "value.h"

typedef enum
{
  OP_CONSTANT,
  OP_NIL,
  OP_TRUE,
  OP_FALSE,
  OP_POP,
  OP_GET_LOCAL,
  OP_SET_LOCAL,
  OP_GET_GLOBAL,
  OP_GET_GLOBAL_FAST,
  OP_DEFINE_GLOBAL,
  OP_SET_GLOBAL,
  OP_EQUAL,
  OP_GREATER,
  OP_LESS,
  OP_ADD,
  OP_SUBTRACT,
  OP_MULTIPLY,
  OP_DIVIDE,
  OP_NOT,
  OP_NEGATE,
  OP_PRINT,
  OP_JUMP,
  OP_JUMP_IF_FALSE,  // Reads the top value from the stack and if false jumps to a placeholder location -- operand:
                     // (0xFFFF) see function: emitJump
  OP_LOOP,
  OP_RETURN,
  OP_FOO
} OpCode;

/*
 * ## Struct: Chunk
 *
 * @brief A chunk is a sequence of bytecode instructions. The basic code segment
 * of for generating bytcode.
 *
 * @param count - how many entries in the array are in use
 * @param capacity - the number of elements in the array that have been
 * allocated
 * @param code - the bytecode instructions in the chunk
 * @param constants - constants associated with the code
 * @param lines - the lines of code in the chunk
 */
typedef struct
{
  int        count;
  int        capacity;
  uint8_t*   code;  // list of bytecode instructions
  int*       lines;
  char*      rleLines;
  ValueArray constants;
} Chunk;

void initChunk(Chunk* chunk);

void freeChunk(Chunk* chunk);
void writeChunk(Chunk* chunk, uint8_t byte, int line);
void writeRleChunk(Chunk* chunk, uint8_t byte);
// void getLine(Chunk* chunk, int offset);
int addConstant(Chunk* chunk, Value value);

#endif