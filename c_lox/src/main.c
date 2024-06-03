#include "chunk.h"
#include "common.h"
#include "debug.h"

int main(int argc, const char* argv[]) {
  Chunk chunk;
  initChunk(&chunk);

  int constant = addConstant(&chunk, 1.2);  // create a constant
  writeChunk(&chunk, OP_CONSTANT, 123);     // write the op code to the chunk
  writeChunk(&chunk, constant, 123);

  writeChunk(&chunk, OP_RETURN, 123);
  disassembleChunk(&chunk, "test chunk");

  printChunk(&chunk);

  freeChunk(&chunk);
  return 0;
}