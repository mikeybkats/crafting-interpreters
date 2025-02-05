#include "vm.h"

#include <stdarg.h>
#include <stdio.h>
#include <string.h>

#include "common.h"
#include "compiler.h"
#include "debug.h"
#include "memory.h"
#include "object.h"

/*
# VM

"This module is eventually going to have a slew of functions and it would be a
chore to pass around a pointer to the VM to all of them. Instead, we declare a
single global VM object. We need only one anyway, and this keeps the code in the
book a little lighter on the page."

"the choice to have a static VM instance is a concession for the book."

"not necessarily a sound engineering choice for a real language implementation"

"It gives better flexability (if explicitly passing a pointer to the VM) for a
VM that's designed to be embedded in other host applications"

"the host app can control when and where memory for the VM is allocated, run
multiple VMs in parallel, ect"

This is a global variable, and as such comes with all the bad things associated
with global variables.
*/
VM vm;

static void resetStack() {
  vm.stackTop = vm.stack;
}

static void runtimeError(const char* format, ...) {
  va_list args;
  va_start(args, format);
  vfprintf(stderr, format, args);
  va_end(args);
  fprintf(stderr, "\n");

  size_t instruction = vm.ip - vm.chunk->code - 1;
  int    line        = vm.chunk->lines[instruction];
  fprintf(stderr, "[line %d] in script\n", line);
  resetStack();
}

void initVM() {
  resetStack();
  vm.objects = NULL;
  initTable(&vm.strings);
}

void freeVM() {
  freeTable(&vm.strings);
  freeObjects();
}

void push(Value value) {
  *vm.stackTop = value;  // stores the value in the array of Values after the
                         // last element in the array
  vm.stackTop++;         // pointer arithmetic
}

Value pop() {
  vm.stackTop--;  // pointer arithmetic
  return *vm.stackTop;
}

/**
 * ## Function: peek
 *
 * @brief Returns the value at the given distance from the top of the stack, but does not pop it from the stack.
 */
static Value peek(int distance) {
  return vm.stackTop[-1 - distance];
}

static bool isFalsey(Value value) {
  return IS_NIL(value) || (IS_BOOL(value) && !AS_BOOL(value));
}

static void concatenate() {
  ObjString* a = AS_STRING(pop());
  ObjString* b = AS_STRING(pop());

  int   length = a->length + b->length;
  char* chars  = ALLOCATE(char, length + 1);
  memcpy(chars, a->chars, a->length);
  memcpy(chars + a->length, b->chars, b->length);
  chars[length] = '\0';

  ObjString* result = takeString(chars, length);

  push(OBJ_VAL(result));
}

/*
 # Run

 "When the interpreter executes a user’s program, it will spend something like
 90% of its time inside run(). It is the beating heart of the VM."

 "Contrast this with all of the complexity and overhead we had in jlox with the
 Visitor pattern for walking the AST."
 */
static InterpretResult run() {
/* READ_BYTE - macro reads the byte currently pointed at by the instruction pointer then advances
the instruction pointer.
*/
#define READ_BYTE() (*vm.ip++)
  /* as soon as the opcode is read from the ip the ip is advanced. Meaning the
   * ip always points to the next byte of code to be used. */

#define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()])

#define BINARY_OP(valueType, op)                      \
  do {                                                \
    if (!IS_NUMBER(peek(0)) || !IS_NUMBER(peek(1))) { \
      runtimeError("Operands must be numbers.");      \
      return INTERPRET_RUNTIME_ERROR;                 \
    }                                                 \
    double b = AS_NUMBER(pop());                      \
    double a = AS_NUMBER(pop());                      \
    push(valueType(a op b));                          \
  } while (false)

  for (;;) {
#ifdef DEBUG_TRACE_EXECUTION
    printf("         ");
    for (Value* slot = vm.stack; slot < vm.stackTop; slot++) {
      // This loop lets us observe the effect of each instruction on the stack.
      printf("[ ");
      printValue(*slot);
      printf(" ]");
    }
    printf("\n");

    disassembleInstruction(vm.chunk,
                           (int)(vm.ip - vm.chunk->code));  // When you subtract two pointers, the
                                                            // result is of type ptrdiff_t, which
                                                            // represents the distance between two
                                                            // pointers, hence the int type cast
#endif

    u_int8_t instruction;

    // This switch statement will become giant to handle all the opcodes
    switch (instruction = READ_BYTE()) {
      case OP_CONSTANT: {
        Value constant = READ_CONSTANT();
        push(constant);
        break;
      }
      case OP_NIL:
        push(NIL_VAL);
        break;
      case OP_TRUE:
        push(BOOL_VAL(true));
        break;
      case OP_FALSE:
        push(BOOL_VAL(false));
        break;
      case OP_EQUAL: {
        Value b = pop();
        Value a = pop();
        push(BOOL_VAL(valuesEqual(a, b)));
        break;
      }
      case OP_GREATER:
        BINARY_OP(BOOL_VAL, >);
        break;
      case OP_LESS:
        BINARY_OP(BOOL_VAL, <);
        break;
      case OP_ADD:
        if (IS_STRING(peek(0)) && IS_STRING(peek(1))) {
          concatenate();
        } else if (IS_NUMBER(peek(0)) && IS_NUMBER(peek(1))) {
          double b = AS_NUMBER(pop());
          double a = AS_NUMBER(pop());
          push(NUMBER_VAL(a + b));
        } else {
          runtimeError("Operands must be two numbers or two strings.");
          return INTERPRET_RUNTIME_ERROR;
        }
        break;
      case OP_SUBTRACT:
        BINARY_OP(NUMBER_VAL, -);
        break;
      case OP_MULTIPLY:
        BINARY_OP(NUMBER_VAL, *);
        break;
      case OP_DIVIDE:
        BINARY_OP(NUMBER_VAL, /);
        break;
      case OP_NOT:
        push(BOOL_VAL(!AS_BOOL(pop())));
        break;
      case OP_NEGATE:
        if (!IS_NUMBER(peek(0))) {
          runtimeError("Operand must be a number.");  // "Lox’s approach to error-handling is rather . . . spare. All
                                                      // errors are fatal and immediately halt the interpreter. There’s
                                                      // no way for user code to recover from an error. If Lox were a
                                                      // real language, this is one of the first things I would remedy."
          return INTERPRET_RUNTIME_ERROR;
        }
        push(NUMBER_VAL(-AS_NUMBER(pop())));
        break;
      case OP_RETURN: {
        printValue(pop());
        printf("\n");
        return INTERPRET_OK;
      }
    }
  }

/*
Undefining these macros explicitly might seem needlessly fastidious, but C tends
to punish sloppy users, and the C preprocessor doubly so.
*/
#undef READ_BYTE
#undef READ_CONSTANT
#undef BINARY_OP
}

/*
## interpret

returns InterpretResult enum

The compiler reports static errors, the VM detects runtime errors, the
interpretter will use this information to know how to set the exit code of the
process.
*/
InterpretResult interpret(const char* source) {
  Chunk chunk;
  initChunk(&chunk);

  // compiler fills chunk with bytecode
  if (!compile(source, &chunk)) {
    // if an error is encountered the chunk is freed and an error is returned
    freeChunk(&chunk);
    return INTERPRET_COMPILE_ERROR;
  }

  vm.chunk = &chunk;
  vm.ip    = vm.chunk->code;

  InterpretResult result = run();

  freeChunk(&chunk);
  return result;
}