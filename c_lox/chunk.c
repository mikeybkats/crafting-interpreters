#include "chunk.h"

#include <stdlib.h>

#include "memory.h"

/*
 * ## initChunk
 *
 * Initializes a chunk in memory, which is the main building block for clox
 * data.
 */
void initChunk(Chunk* chunk) {
  chunk->count = 0;
  chunk->capacity = 0;
  chunk->code = NULL;
}

/*
 * ## writeChunk
 *
 * Writes a chunk to memory.
 */
void writeChunk(Chunk* chunk, uint8_t byte) {
  // check to see if the array has adequate capacity
  if (chunk->capacity < chunk->count + 1) {
    // if it does not, then grown the array
    int oldCapacity = chunk->capacity;
    chunk->capacity = GROW_CAPACITY(oldCapacity);
    chunk->code =
        GROW_ARRAY(uint8_t, chunk->code, oldCapacity, chunk->capacity);
  }

  chunk->code[chunk->count] = byte;
  chunk->count++;
}

/*
 * ## freeChunk
 *
 * Free a chunk from memory
 */
void freeChunk(Chunk* chunk) {
  FREE_ARRAY(uint8_t, chunk->code, chunk->capacity);
  initChunk(chunk);
}