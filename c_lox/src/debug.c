#include "debug.h"

#include <stdio.h>

#include "value.h"
#include "vm.h"

/*
 * ## disassembleChunk
 *
 * Given a chunk of machine code, it will print all the instructions in it
 */
void disassembleChunk(Chunk* chunk, char* name) {
  printf("== %s ==\n", name);
  printf("offset line opcode             operand   constant\n");
  printf("------ ---- ------------------ --------- --------\n");

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

/*
 * ## constantInstruction
 *
 * @brief Prints the name of the instruction and the constant value.
 *
 * @param name the name of the instruction.
 * @param chunk the Chunk containing the instruction.
 * @param offset the offset of the current instruction.
 */
static int constantInstruction(const char* name, Chunk* chunk, int offset) {
  uint8_t constantIndex = chunk->code[offset + 1];     // the constantIndex is the operand for the instruction
  printf(" %-18s   0x%02X   '", name, constantIndex);  // print the index of the constant
  printValue(chunk->constants.values[constantIndex]);  // print the constant value
  printf("'\n");

  return offset + 2;
}

static int globalInstruction(const char* name, Chunk* chunk, int offset) {
  uint8_t globalIndex = chunk->code[offset + 1];     // the constantIndex is the operand for the instruction
  printf(" %-18s   0x%02X   '", name, globalIndex);  // print the index
  printValue(vm.globalsCache[globalIndex].value);    // print the  value
  printf("'\n");

  return offset + 2;
}

/*
 * ## simpleInstruction
 *
 * @brief Prints the name of the instruction and returns the offset of the next
 * instruction.
 *
 * @param name the name of the instruction.
 * @param offset the offset of the current instruction.
 */
static int simpleInstruction(const char* name, int offset) {
  printf(" %-18s\n", name);
  return offset + 1;
}

static int byteInstruction(const char* name, Chunk* chunk, int offset) {
  uint8_t slot = chunk->code[offset + 1];
  printf("%-18s   0x%02X\n", name, slot);
  return offset + 2;
}

static int jumpInstruction(const char* name, int sign, Chunk* chunk, int offset) {
  uint16_t jump = (uint16_t)(chunk->code[offset + 1] << 8);
  jump |= chunk->code[offset + 2];
  printf(" %-18s   0x%04X  token: %d -> token: %d\n", name, jump, offset, offset + 3 + sign * jump);
  return offset + 3;
}

/*
 * ## disassembleInstruction
 *
 * @brief Prints the offset. Prints the line number. Prints the opcode of the next instruction. Returns the offset of
 * the next instruction.
 *
 * @param chunk
 * @param offset bytecode offset from the beginning of a code array to access a
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

    case OP_SET_GLOBAL:
      return constantInstruction("OP_SET_GLOBAL", chunk, offset);

    case OP_EQUAL:
      return simpleInstruction("OP_EQUAL", offset);

    case OP_GET_GLOBAL:
      return constantInstruction("OP_GET_GLOBAL", chunk, offset);

    case OP_GET_GLOBAL_FAST:
      return globalInstruction("OP_GET_GLOBAL_FAST", chunk, offset);

    case OP_DEFINE_GLOBAL:
      return constantInstruction("OP_DEFINE_GLOBAL", chunk, offset);

    case OP_GREATER:
      return simpleInstruction("OP_GREATER", offset);

    case OP_LESS:
      return simpleInstruction("OP_LESS", offset);

    case OP_ADD:
      return simpleInstruction("OP_ADD", offset);

    case OP_SUBTRACT:
      return simpleInstruction("OP_SUBTRACT", offset);

    case OP_MULTIPLY:
      return simpleInstruction("OP_MULTIPLY", offset);

    case OP_DIVIDE:
      return simpleInstruction("OP_DIVIDE", offset);

    case OP_NOT:
      return simpleInstruction("OP_NOT", offset);

    case OP_NEGATE:
      return simpleInstruction("OP_NEGATE", offset);

    case OP_PRINT:
      return simpleInstruction("OP_PRINT", offset);

    case OP_JUMP:
      return jumpInstruction("OP_JUMP", 1, chunk, offset);

    case OP_JUMP_IF_FALSE:
      return jumpInstruction("OP_JUMP_IF_FALSE", 1, chunk, offset);

    case OP_LOOP:
      return jumpInstruction("OP_LOOP", -1, chunk, offset);

    case OP_RETURN:
      return simpleInstruction("OP_RETURN", offset);

    case OP_POP:
      return simpleInstruction("OP_POP", offset);

    case OP_GET_LOCAL:
      return byteInstruction("OP_GET_LOCAL", chunk, offset);

    case OP_SET_LOCAL:
      return byteInstruction("OP_SET_LOCAL", chunk, offset);

    default:
      printf("Unknown opcode %d\n", opcode);
      return offset + 1;
  }
}