#include "vm.h"

#include "common.h"

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
## interpret

returns InterpretResult enum

The compiler reports static errors, the VM detects runtime errors, the
interpretter will use this information to know how to set the exit code of the
process.
*/
InterpretResult interpret(Chunk* chunk) {
  vm.chunk = chunk;
  vm.ip = vm.chunk->code;
  return run();
}