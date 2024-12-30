#include "debug.h"

#include <stdio.h>

#include "value.h"

/*
 * ## disassembleChunk
 *
 * Given a chunk of machine code, it will print all the instructions in it
 */
void disassembleChunk(Chunk* chunk, char* name) {
  printf("== %s ==\n", name);
  printf("offset line opcode             constant\n");
  printf("------ ---- ------------------ --------\n");

  for (int offset = 0; offset < chunk->count;) {
    offset = disassembleInstruction(chunk, offset);
  }
}

void printChunk(Chunk* chunk) {
  printf("\n\n== chunk ==\n");

  printf("count: %d\n", chunk->count);
  printf("capacity: %d\n", chunk->capacity);

  printf("code:  [");
  for (int i = 0; i < chunk->count; i++) {
    printf("%d", chunk->code[i]);

    if (i < chunk->count - 1) {
      printf(", ");
    }
  }
  printf("]\n");

  printf("lines: [");
  for (int i = 0; i < chunk->count; i++) {
    printf("%d", chunk->lines[i]);
    if (i < chunk->count - 1) {
      printf(", ");
    }
  }
  printf("]\n");
}

static int constantInstruction(const char* name, Chunk* chunk, int offset) {
  uint8_t constantIndex = chunk->code[offset + 1];
  printf(" %-16s", name);                              // print the opcode name of the instruction
  printf("%4d: '", constantIndex);                     // print the index of the constant
  printValue(chunk->constants.values[constantIndex]);  // print the constant value
  printf("'\n");

  return offset + 2;
}

static int simpleInstruction(const char* name, int offset) {
  printf(" %s\n", name);
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
  printf("%04D  ", offset);  // print the offset

  if (offset > 0 && chunk->lines[offset] == chunk->lines[offset - 1]) {
    printf("   | ");
  } else {
    printf("%4D ", chunk->lines[offset]);  // print the line number
  }

  // read a single byte from the bytecode at the given offset
  uint8_t opcode = chunk->code[offset];

  switch (opcode) {
    case OP_CONSTANT:
      return constantInstruction("OP_CONSTANT", chunk, offset);

    case OP_NIL:
      return simpleInstruction("OP_NIL", offset);

    case OP_TRUE:
      return simpleInstruction("OP_TRUE", offset);

    case OP_FALSE:
      return simpleInstruction("OP_FALSE", offset);

    case OP_ADD:
      return simpleInstruction("OP_ADD", offset);

    case OP_SUBTRACT:
      return simpleInstruction("OP_SUBTRACT", offset);

    case OP_MULTIPLY:
      return simpleInstruction("OP_MULTIPLY", offset);

    case OP_DIVIDE:
      return simpleInstruction("OP_DIVIDE", offset);

    case OP_NEGATE:
      return simpleInstruction("OP_NEGATE", offset);

    case OP_RETURN:
      return simpleInstruction("OP_RETURN", offset);

    default:
      printf("Unknown opcode %d\n", opcode);
      return offset + 1;
  }
}