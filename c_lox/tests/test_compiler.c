#include "test_compiler.h"

#include "../src/chunk.h"
#include "../src/compiler.h"
#include "../src/compiler_internal.h"
#include "../src/scanner.h"
#include "../src/scanner_internal.h"
#include "unity/src/unity.h"

// Global test variables matching compiler.c globals
static Chunk       testChunk;  // matches: Chunk* compilingChunk
static const char* testSource;

void setUpCompiler(void) {
  printf("setUpCompiler\n");

  testSource = "1 + 2 * 3";
  initVM();
  initScanner(testSource);
  initChunk(&testChunk);
  compilingChunk = testChunk;
}

void tearDownCompiler(void) {
  printf("\ntearDownCompiler\n");
  freeChunk(&testChunk);
  freeVM();
}

void run_compiler_tests(void) {
  printf("\n\033[0;31mTest compiler ---\033[0m \n");
}
