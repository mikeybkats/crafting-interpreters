#ifndef clox_compiler_h
#define clox_compiler_h

#include "vm.h"

// using the const keyword provides safety, and dissalows writing to the source argument
bool compile(const char* source, Chunk* chunk);

#endif