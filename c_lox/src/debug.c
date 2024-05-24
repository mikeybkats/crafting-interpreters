#include "debug.h"

#include <stdio.h>

/*
 * ## disassembleChunk
 *
 * Given a chunk of machine code, it will print all the instructions in it
 */
void disassembleChunk(Chunk* chunk, char* name) {
  printf("== %s ==\n", name);

  for (int offset = 0; offset < chunk->count;) {
    offset = disassembleInstruction(chunk, offset);
  }
}

static int simpleInstruction(const char* name, int offset) {
  printf("%s\n", name);
  return offset + 1;
}

/*
 * ## disassembleInstruction
 *
 * @brief Returns the offset of the next instruction.
 *
 * @param chunk
 * @param offset the int distance from the beginning of a code array to access a
 * given piece of code.
 */
int disassembleInstruction(Chunk* chunk, int offset) {
  printf("%04D", offset);

  // read a single byte from the bytecode at the given offset
  uint8_t opcode = chunk->code[offset];

  switch (opcode) {
    case OP_CONSTANT:
      // return constantInstruction("OP_CONSTANT", chunk, offset);

    case OP_RETURN:
      return simpleInstruction("OP_RETURN", offset);

    default:
      printf("Unknown opcode %d\n", opcode);
      return offset + 1;
  }
}

static int constantInstruction(const char* name, Chunk* chunk, int offset) {
  uint8_t constant = chunk->code[offset + 1];
  printf("%-16s %4d '", name, constant);
  // printValue(chunk->constants.values[constant]);
  printf("'\n");
}