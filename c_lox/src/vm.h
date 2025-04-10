#ifndef clox_vm_h
#define clox_vm_h

#include "chunk.h"
#include "table.h"
#include "value.h"

#define STACK_MAX 256

// "you hand the virtual machine a chunk of code, and it runs it."

typedef struct
{
  char* name;
  Value value;
  int   index;
} CachedGlobal;

typedef struct
{
  // the chunk of code to execute "the VM will gradually acquire more state"
  Chunk* chunk;
  // ip points to the bytecode inside the chunk
  uint8_t* ip;  // "ip stands for instruction pointer. Pointing to the
                // bytecode array inside the chunk, faster than looking up
                // the bytecode by index."
  Value stack[STACK_MAX];
  // stack top points to one past the top item in the stack
  Value* stackTop;  // "The pointer points at the array element just past the
  // element containing the top value on the stack. That seems
  // a little odd, but almost every implementation does this."
  Table strings;  // the interned strings for our interpreter
  Table globals;  // a hash table of the global variables for our interpreter
  Table globalConsts;
  CachedGlobal*
      globalsCache;  // The first time a global is defined, it gets added to both the hash table AND assigned the
  // next available index in the array
  uint8_t globalsCacheCount;

  Obj* objects;
} VM;

typedef enum
{
  INTERPRET_OK,
  INTERPRET_COMPILE_ERROR,
  INTERPRET_RUNTIME_ERROR
} InterpretResult;

extern VM vm;

void            initVM();
void            freeVM();
InterpretResult interpret(const char* source);

/*
A proper stack implementation needs to:

a) Keep track of the current top of the stack.
b) Allocate enough memory for multiple elements.
c) Resize the allocated memory when needed.
d) Provide both push and pop operations.
*/
void  push(Value value);
Value pop();

#endif
