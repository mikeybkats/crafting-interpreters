#include "vm.h"

#include <stdio.h>

#include "common.h"
#include "compiler.h"
#include "debug.h"

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

static void resetStack() { vm.stackTop = vm.stack; }

void initVM() { resetStack(); }

void freeVM() {}

void push(Value value) {
  *vm.stackTop = value;  // stores the value in the array of Values after the
                         // last element in the array
  vm.stackTop++;         // pointer arithmetic
}

Value pop() {
  vm.stackTop--;  // pointer arithmetic
  return *vm.stackTop;
}

/*
 # Run

 "When the interpreter executes a user’s program, it will spend something like
 90% of its time inside run(). It is the beating heart of the VM."

 "Contrast this with all of the complexity and overhead we had in jlox with the
 Visitor pattern for walking the AST."
 */
static InterpretResult run() {
/* READ_BYTE - macro reads the byte currently pointed at by the ip then advances
the instruction pointer.
*/
#define READ_BYTE() (*vm.ip++)
  /* as soon as the opcode is read from the ip the ip is advanced. Meaning the
   * ip always points to the next byte of code to be used. */

#define BINARY_OP(op) \
  do {                \
    double b = pop(); \
    double a = pop(); \
    push(a op b);     \
  } while (false)

#define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()])

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

    disassembleInstruction(
        vm.chunk,
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
      case OP_ADD:
        BINARY_OP(+);
        break;
      case OP_SUBTRACT:
        BINARY_OP(-);
        break;
      case OP_MULTIPLY:
        BINARY_OP(*);
        break;
      case OP_DIVIDE:
        BINARY_OP(/);
        break;
      case OP_NEGATE:
        push(-pop());
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
  compile(source);
  return INTERPRET_OK;
}