#ifndef clox_chunk_h
#define clox_chunk_h

#include "common.h"
#include "memory.h"
#include "value.h"

typedef enum {
  OP_CONSTANT,
  OP_RETURN,
} OpCode;

/*
 * ## Struct: Chunk
 *
 * @brief The basic code segment of for generating bytcode
 *
 * @param count how many entries in the array are in use
 *
 * @param capacity the number of elements in the array that have been
 * allocated
 *
 * @param code the code in the chunk
 *
 * @param constants constants associated with the code
 */
typedef struct {
  int count;
  int capacity;
  uint8_t* code;
  int* lines;
  ValueArray constants;
} Chunk;

void initChunk(Chunk* chunk);
void freeChunk(Chunk* chunk);
void writeChunk(Chunk* chunk, uint8_t byte, int line);
int addConstant(Chunk* chunk, Value value);

#endif