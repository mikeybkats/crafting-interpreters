#include "chunk.h"

#include <stdio.h>
#include <stdlib.h>

#include "memory.h"

/*
 * ## initChunk
 *
 * @brief Initializes a chunk in memory, which is the main building block for
 * clox data.
 *
 * Q: What should the max capacity for a chunk be?
 */
void initChunk(Chunk* chunk) {
  chunk->count    = 0;
  chunk->capacity = 0;
  chunk->code     = NULL;
  chunk->lines    = NULL;
  chunk->rleLines = NULL;
  initValueArray(&chunk->constants);
}

/*
 * ## writeChunk
 *
 * @brief Writes to a chunk of memory.
 *
 * @param chunk the initialized Chunk to write to.
 * @param byte the code to write to the chunk. Can be an OpCode or a constant.
 * @param line the line number of the code
 */
void writeChunk(Chunk* chunk, uint8_t byte, int line) {
  // check to see if the array has adequate capacity
  if (chunk->capacity < chunk->count + 1) {
    // if it does not, then grown the array
    int oldCapacity = chunk->capacity;
    chunk->capacity = GROW_CAPACITY(oldCapacity);
    chunk->code     = GROW_ARRAY(uint8_t, chunk->code, oldCapacity, chunk->capacity);
    // Grow array runs a reallocate function
    // which under the hood is just a call of
    // realloc, unless the new capacity is 0.
    // In which case it frees the memory.
    chunk->lines = GROW_ARRAY(int, chunk->lines, oldCapacity, chunk->capacity);
  }

  chunk->code[chunk->count]  = byte;
  chunk->lines[chunk->count] = line;
  chunk->count++;
}

/*
 * ## writeRleChunk
 *
 * Our encoding of line information is hilariously wasteful of memory. Given
 * that a series of instructions often correspond to the same source line, a
 * natural solution is something akin to run-length encoding of the line
 * numbers.
 *
 * Devise an encoding that compresses the line information for a series of
 * instructions on the same line. Change writeChunk() to write this compressed
 * form, and implement a getLine() function that, given the index of an
 * instruction, determines the line where the instruction occurs.
 *
 * Hint: It’s not necessary for getLine() to be particularly efficient. Since it
 * is called only when a runtime error occurs, it is well off the critical path
 * where performance matters.
 */
void writeRleChunk(Chunk* chunk, uint8_t byte) {
  // check to see if the array has adequate capacity
  if (chunk->capacity < chunk->count + 1) {
    // if it does not, then grown the array
    int oldCapacity = chunk->capacity;
    chunk->capacity = GROW_CAPACITY(oldCapacity);
    chunk->code     = GROW_ARRAY(uint8_t, chunk->code, oldCapacity, chunk->capacity);
    // chunk->lines = rle_encode
  }

  chunk->code[chunk->count] = byte;
  // chunk->lines[chunk->count] = line;
  chunk->count++;
}

/*
 * ## getLine()
 *
 * @brief a function that, given the index of an instruction, determines
 * the line where the instruction occurs.
 */
// void getLine(Chunk* chunk, int offset) {}

/*
 * ## freeChunk
 *
 * @brief Free a chunk from memory
 */
void freeChunk(Chunk* chunk) {
  FREE_ARRAY(uint8_t, chunk->code, chunk->capacity);
  FREE_ARRAY(int, chunk->lines, chunk->capacity);
  freeValueArray(&chunk->constants);  // free the constants when the chunk is freed
  initChunk(chunk);
}

/*
 * ## addConstant
 *
 * @brief Adds a value to the end of the chunk's array of constants. If the value exists it will replace that value.
 *
 * @param chunk the Chunk to add the value to
 * @param value the Value to add to the chunk
 *
 * @returns the index of the constants array
 */
int addConstant(Chunk* chunk, Value value) {
  // search through existing constants to see if the value already exists
  for (int i = 0; i < chunk->constants.count; i++) {
    if (valuesEqual(chunk->constants.values[i], value)) {
      return i;
    }
  }

  writeValueArray(&chunk->constants, value);
  return chunk->constants.count - 1;
}