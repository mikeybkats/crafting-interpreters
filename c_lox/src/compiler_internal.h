#ifndef clox_compiler_internal_h
#define clox_compiler_internal_h

#include "compiler.h"

// Expose the internal variables from compiler.c
extern Parser parser;
extern Chunk  compilingChunk;

#ifdef DEBUG_TEST
void   test_advance();
Chunk* currentChunk();
#endif

#endif