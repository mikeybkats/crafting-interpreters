#include "test_chunk.h"
#include "test_compiler.h"
#include "test_rle.h"
#include "test_value.h"
#include "unity/src/unity.h"

void setUp(void) { setUpCompiler(); }

void tearDown(void) { tearDownCompiler(); }

int main(void) {
  UNITY_BEGIN();

  // Run all test suites
  run_chunk_tests();
  // run_value_tests();
  // run_rle_tests();
  run_compiler_tests();

  return UNITY_END();
}