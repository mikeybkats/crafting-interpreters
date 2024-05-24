#include "../src/chunk.h"
#include "unity/src/unity.h"

void test_initChunk(void) {
  printf("Test initChunk\n");
  Chunk chunk;
  initChunk(&chunk);

  TEST_ASSERT_EQUAL_INT_MESSAGE(0, chunk.count, "Count should be 0");
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, chunk.capacity, "Capacity should be 0");
  TEST_ASSERT_NULL_MESSAGE(chunk.code, "Code should be NULL");
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, chunk.constants.count,
                                "Constants count should be 0");
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, chunk.constants.capacity,
                                "Constants capacity should be 0");

  freeChunk(&chunk);
}
