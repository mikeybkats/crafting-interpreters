#include "vm.h"

#include <stdio.h>

#include "common.h"
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

void initVM() {}

void freeVM() {}

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

#define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()])

  for (;;) {
#ifdef DEBUG_TRACE_EXECUTION
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
        printf("\nOP_CONSTANT: \n");
        printValue(constant);
        printf("\n");
        break;
      }
      case OP_RETURN: {
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
}

/*
## interpret

returns InterpretResult enum

The compiler reports static errors, the VM detects runtime errors, the
interpretter will use this information to know how to set the exit code of the
process.
*/
InterpretResult interpret(Chunk* chunk) {
  vm.chunk = chunk;
  vm.ip =
      vm.chunk
          ->code;  // the location of the instruction currently being executed
  return run();
}