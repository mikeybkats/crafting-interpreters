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

#define GLOBALS_CACHE_DEFAULT_SIZE 100
// TODO: Fall back to hash table lookups for additional globals when this limit is reached

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

void initGlobalsCache() {
  vm.globalsCache = ALLOCATE(CachedGlobal, GLOBALS_CACHE_DEFAULT_SIZE);

  for (int i = 0; i < GLOBALS_CACHE_DEFAULT_SIZE; i++) {
    vm.globalsCache[i].name  = NULL;
    vm.globalsCache[i].value = NIL_VAL;
    vm.globalsCache[i].index = i;
  }
}

void initVM() {
  resetStack();
  vm.objects = NULL;
  initTable(&vm.globals);
  initTable(&vm.strings);

  vm.globalsCacheCount = GLOBALS_CACHE_DEFAULT_SIZE;
  initGlobalsCache();
}

void freeVM() {
  freeTable(&vm.globals);
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

// static bool isFalsey(Value value) {
//   return IS_NIL(value) || (IS_BOOL(value) && !AS_BOOL(value));
// }

static void concatenate() {
  ObjString* b = AS_STRING(pop());
  ObjString* a = AS_STRING(pop());

  int   length = a->length + b->length;
  char* chars  = ALLOCATE(char, length + 1);
  memcpy(chars, a->chars, a->length);
  memcpy(chars + a->length, b->chars, b->length);
  chars[length] = '\0';

  ObjString* result = takeString(chars, length);

  push(OBJ_VAL(result));
}

/**
 * ## patchGlobalToCache
 *
 * Takes the name, value and globalIndex and replaces all future instructions to get the global from the globals cache
 * which will provide faster access than the globals table.
 *
 * #### Performance Tradeoff Analysis
 *
 * One-time cost: The scanning is a one-time cost per global variable per execution
 * Ongoing benefit: Every access to the variable after patching is much faster
 * Amortized cost: As the program runs longer, the initial cost becomes increasingly negligible
 *
 * If a variable is accessed 1,000 times in a program, paying the cost of scanning once to optimize the other 999
 * accesses is a clear win.
 *
 * #### Potential Improvements
 *
 * If you were concerned about the scanning cost for very large programs, you could consider:
 *
 * Lazy patching: Only patch a small window ahead of the current instruction
 * Compilation-time analysis: Identify all global variable accesses during compilation
 * Patching threshold: Only patch if a global is used more than X times
 */
void patchGlobalToCache(ObjString* name, Value* value, int globalIndex) {
  // OP_GET_GLOBAL should only ever run once per global. After it runs then all subsequent global gets will go
  // through the cache. Create an entry for the globals cache
  CachedGlobal* global = &vm.globalsCache[globalIndex];
  global->index        = globalIndex;
  global->name         = AS_CSTRING(OBJ_VAL(name));
  global->value        = *value;

  for (int i = (int)(vm.ip - vm.chunk->code); i < vm.chunk->count - 1; i++) {
    uint8_t constantIndex = vm.chunk->code[i + 1];
    Value   value         = vm.chunk->constants.values[constantIndex];
    if (value.type == VAL_OBJ && vm.chunk->code[i] == OP_GET_GLOBAL && AS_CSTRING(value) == global->name) {
      vm.chunk->code[i]     = OP_GET_GLOBAL_FAST;
      vm.chunk->code[i + 1] = global->index;
    }
  }
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
#define READ_STRING()   AS_STRING(READ_CONSTANT())

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

    u_int8_t instruction;  // every bytecode instruction modifies the stack

    // printf("OP_CODE before READ_BYTE: %hhu\n", *vm.ip);
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
      case OP_POP:
        pop();
        break;
      case OP_GET_GLOBAL: {
        ObjString* name = READ_STRING();
        Value      value;
        int        globalIndex;

        if (!tableGet(&vm.globals, &OBJ_VAL(name), &value, &globalIndex)) {
          runtimeError("Undefined variable '%s'.", name->chars);
          return INTERPRET_RUNTIME_ERROR;
        }

        patchGlobalToCache(name, &value, globalIndex);

        push(value);
        break;
      }

      case OP_GET_GLOBAL_FAST: {
        int   index = READ_BYTE();
        Value value = vm.globalsCache[index].value;

        printf("Getting global fast -- index: %d value: ", index);
        printValue(value);
        printf("\n");

        push(value);

        break;
      }

      case OP_DEFINE_GLOBAL: {
        // when global is defined set it to the table and the vm globalsCache
        ObjString* name = READ_STRING();

        tableSet(&vm.globals, &OBJ_VAL(name), peek(0));
        pop();
        break;
      }
      case OP_SET_GLOBAL: {
        ObjString* name = READ_STRING();

        if (tableSet(&vm.globals, &OBJ_VAL(name), peek(0))) {
          tableDelete(&vm.globals, &OBJ_VAL(name));
          runtimeError("Undefined variable '%s'.", name->chars);
          return INTERPRET_RUNTIME_ERROR;
        }
        break;
      }

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
      case OP_PRINT:
        printValue(pop());
        printf("\n");
        break;
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
#undef READ_STRING
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