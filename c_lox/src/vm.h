#ifndef clox_vm_h
#define clox_vm_h

#include "chunk.h"

// you hand the virtual machine a chunk of code, and it runs it.
typedef struct {
  // the VM will gradually acquire more state
  Chunk* chunk;  // the chunk to execute
  uint8_t* ip;   // ip stands for instruction pointer
} VM;

typedef enum {
  INTERPRET_OK,
  INTERPRET_COMPILE_ERROR,
  INTERPRET_RUNTIME_ERROR
} InterpretResult;

void initVM();
void freeVM();
InterpretResult interpret(Chunk* chunk);

#endif
