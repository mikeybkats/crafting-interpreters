#include "test_chunk.h"
#include "test_value.h"
#include "unity/src/unity.h"

void setUp(void) { /* General initialization */ }
void tearDown(void) { /* General cleanup */ }

int main(void) {
  UNITY_BEGIN();

  // test chunk.c
  RUN_TEST(test_initChunk);

  // test value.c
  RUN_TEST(test_initValueArray);
  RUN_TEST(test_writeValueArray);
  RUN_TEST(test_freeValueArray);

  return UNITY_END();
}