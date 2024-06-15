#include "../src/value.h"
#include "unity/src/unity.h"

void test_initValueArray(void) {
  printf("Test initValueArray --- ");

  ValueArray array;
  initValueArray(&array);

  TEST_ASSERT_EQUAL_INT_MESSAGE(0, array.capacity, "Capacity should be 0");
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, array.count, "Count should be 0");
  TEST_ASSERT_NULL_MESSAGE(array.values, "Values should be NULL");
}

void test_writeValueArray(void) {
  printf("Test writeValueArray --- ");

  ValueArray array;
  initValueArray(&array);

  Value value = 1.2;
  writeValueArray(&array, value);

  TEST_ASSERT_EQUAL_INT_MESSAGE(1, array.count, "Count should be 1");
  TEST_ASSERT_EQUAL_INT_MESSAGE(8, array.capacity, "Capacity should be 8");
  TEST_ASSERT_EQUAL_FLOAT_MESSAGE(value, array.values[0],
                                  "Value should be 1.2");

  freeValueArray(&array);
}

void test_freeValueArray(void) {
  printf("Test writeValueArray --- ");

  ValueArray array;
  initValueArray(&array);

  Value value = 1.2;
  writeValueArray(&array, value);

  freeValueArray(&array);

  TEST_ASSERT_EQUAL_INT_MESSAGE(0, array.capacity, "Capacity should be 0");
}