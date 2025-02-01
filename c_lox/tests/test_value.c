#include "test_value.h"

#include "../src/value.h"
#include "unity/src/unity.h"

void test_initValueArray(void) {
  printf("\n\033[0;31mTest initValueArray ---\033[0m \n");

  ValueArray array;
  initValueArray(&array);

  TEST_ASSERT_EQUAL_INT_MESSAGE(0, array.capacity, "Capacity should be 0");
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, array.count, "Count should be 0");
  TEST_ASSERT_NULL_MESSAGE(array.values, "Values should be NULL");
}

void test_writeValueArray(void) {
  printf("\n\033[0;31mTest writeValueArray ---\033[0m \n");

  ValueArray array;
  initValueArray(&array);

  Value value = NUMBER_VAL(1.2);
  writeValueArray(&array, value);

  TEST_ASSERT_EQUAL_INT_MESSAGE(1, array.count, "Count should be 1");
  TEST_ASSERT_EQUAL_INT_MESSAGE(8, array.capacity, "Capacity should be 8");
  // TEST_ASSERT_EQUAL_FLOAT_MESSAGE(value, array.values[0], "Value should be 1.2");

  freeValueArray(&array);
}

// void test_freeValueArray(void) {
//   printf("\n\033[0;31mTest freeValueArray ---\033[0m \n");

//   ValueArray array;
//   initValueArray(&array);

//   Value value = 1.2;
//   writeValueArray(&array, value);

//   freeValueArray(&array);

//   TEST_ASSERT_EQUAL_INT_MESSAGE(0, array.capacity, "Capacity should be 0");
// }

void run_value_tests(void) {
  RUN_TEST(test_initValueArray);
  RUN_TEST(test_writeValueArray);
  // RUN_TEST(test_freeValueArray);
}