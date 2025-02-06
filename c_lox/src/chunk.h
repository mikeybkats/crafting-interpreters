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
  OP_EQUAL,
  OP_GREATER,
  OP_LESS,
  OP_ADD,
  OP_SUBTRACT,
  OP_MULTIPLY,
  OP_DIVIDE,
  OP_NOT,
  OP_NEGATE,
  OP_RETURN,
} OpCode;

/*
 * ## Struct: Chunk
 *
 * @brief A chunk is a sequence of bytecode instructions. The basic code segment
 * of for generating bytcode.
 *
 * @param count how many entries in the array are in use
 * @param capacity the number of elements in the array that have been
 * allocated
 * @param code the code in the chunk
 * @param constants constants associated with the code
 * @param lines the lines of code in the chunk
 */
typedef struct
{
  int        count;
  int        capacity;
  uint8_t*   code;
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