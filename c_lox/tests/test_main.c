// #include "test_chunk.h"
// #include "test_compiler.h"
// #include "test_rle.h"
// #include "test_value.h"
#include "test_object.h"
#include "unity/src/unity.h"

void setUp(void) {
}

void tearDown(void) {
}

int main(void) {
  UNITY_BEGIN();

  run_object_tests();
  // run_rle_tests();
  // run_chunk_tests();
  // run_value_tests();

  return UNITY_END();
}