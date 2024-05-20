#include "tests/unity/src/unity.h"

void setUp(void) { /* General initialization */ }
void tearDown(void) { /* General cleanup */ }

void test_function2(void) {
  int val = 5;
  TEST_ASSERT_EQUAL_INT_MESSAGE(5, val, "Not five? Not alive!");
}

int main(void) {
  // implement tests
  UNITY_BEGIN();
  RUN_TEST(test_function2);
  return UNITY_END();
}