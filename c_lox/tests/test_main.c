#include "../src/chunk.h"
#include "test_chunk.h"
#include "unity/src/unity.h"

void setUp(void) { /* General initialization */ }
void tearDown(void) { /* General cleanup */ }

void test_function2(void) {
  printf("Test function 2\n");
  int val = 5;
  TEST_ASSERT_EQUAL_INT_MESSAGE(5, val, "Not five? Not alive!");
}

int main(void) {
  // implement tests
  UNITY_BEGIN();
  RUN_TEST(test_function2);
  RUN_TEST(test_initChunk);
  return UNITY_END();
}