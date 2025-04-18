#include "test_chunk.h"

#include "../src/chunk.h"
#include "unity/src/unity.h"

void setUpChunk(void) { /* General initialization */
}
void tearDownChunk(void) { /* General cleanup */
}

void test_initChunk(void) {
  // printf("\n\033[0;31mTest initChunk ---\033[0m \n");
  Chunk chunk;
  initChunk(&chunk);

  TEST_ASSERT_EQUAL_INT_MESSAGE(0, chunk.count, "Count should be 0");
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, chunk.capacity, "Capacity should be 0");
  TEST_ASSERT_NULL_MESSAGE(chunk.code, "Code should be NULL");
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, chunk.constants.count, "Constants count should be 0");
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, chunk.constants.capacity, "Constants capacity should be 0");

  freeChunk(&chunk);
}

void test_writeChunk(void) {
  Chunk chunk;
  initChunk(&chunk);

  // 1 + 1
  writeChunk(&chunk, 1, 1);
  writeChunk(&chunk, OP_ADD, 1);
  writeChunk(&chunk, 1, 1);

  TEST_ASSERT_EQUAL_INT_MESSAGE(3, chunk.count, "Chunk should have 3 instruction");
  TEST_ASSERT_EQUAL_INT_MESSAGE(1, chunk.code[0], "First instruction should be OP_CONSTANT");
  TEST_ASSERT_EQUAL_INT_MESSAGE(OP_ADD, chunk.code[1], "Second instruction should be OP_ADD");
  TEST_ASSERT_EQUAL_INT_MESSAGE(1, chunk.code[2], "Third instruction should be OP_CONSTANT");
  TEST_ASSERT_EQUAL_INT_MESSAGE(1, chunk.lines[0], "Line 1 should be 1");
  TEST_ASSERT_EQUAL_INT_MESSAGE(1, chunk.lines[1], "Line 1 should be 1");
  TEST_ASSERT_EQUAL_INT_MESSAGE(1, chunk.lines[2], "Line 1 should be 1");

  freeChunk(&chunk);
  initChunk(&chunk);

  // 1 + 1 + 1
  writeChunk(&chunk, 1, 2);
  writeChunk(&chunk, OP_ADD, 2);
  writeChunk(&chunk, 1, 2);
  writeChunk(&chunk, OP_ADD, 2);
  writeChunk(&chunk, 1, 2);

  TEST_ASSERT_EQUAL_INT_MESSAGE(5, chunk.count, "Chunk should have 5 instruction");
  TEST_ASSERT_EQUAL_INT_MESSAGE(1, chunk.code[0], "First instruction should be OP_CONSTANT");
  TEST_ASSERT_EQUAL_INT_MESSAGE(OP_ADD, chunk.code[1], "Second instruction should be OP_ADD");
  TEST_ASSERT_EQUAL_INT_MESSAGE(1, chunk.code[2], "Third instruction should be OP_CONSTANT");
  TEST_ASSERT_EQUAL_INT_MESSAGE(OP_ADD, chunk.code[3], "Fourth instruction should be OP_ADD");
  TEST_ASSERT_EQUAL_INT_MESSAGE(1, chunk.code[4], "Fifth instruction should be OP_CONSTANT");
  TEST_ASSERT_EQUAL_INT_MESSAGE(2, chunk.lines[1], "should be 2");
  TEST_ASSERT_EQUAL_INT_MESSAGE(2, chunk.lines[2], "should be 2");
  TEST_ASSERT_EQUAL_INT_MESSAGE(2, chunk.lines[3], "should be 2");
  TEST_ASSERT_EQUAL_INT_MESSAGE(2, chunk.lines[4], "should be 2");

  freeChunk(&chunk);
}

void test_freeChunk(void) {
  Chunk chunk;
  initChunk(&chunk);

  writeChunk(&chunk, 1, 1);
  writeChunk(&chunk, OP_ADD, 1);
  writeChunk(&chunk, 1, 1);

  freeChunk(&chunk);

  TEST_ASSERT_EQUAL_INT_MESSAGE(0, chunk.count, "Chunk should have 0 instruction");
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, chunk.capacity, "Chunk should have 0 capacity");
  TEST_ASSERT_NULL_MESSAGE(chunk.code, "Code should be NULL");
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, chunk.constants.count, "Constants count should be 0");
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, chunk.constants.capacity, "Constants capacity should be 0");
}

void test_addConstant(void) {
  Chunk chunk;
  initChunk(&chunk);

  // Test adding a number constant
  Value numValue = NUMBER_VAL(1.2);
  int   numIndex = addConstant(&chunk, numValue);
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, numIndex, "First constant should have index 0");
  TEST_ASSERT_EQUAL_MESSAGE(1.2, AS_NUMBER(chunk.constants.values[0]), "First constant should be 1.2");

  // Test adding a boolean constant
  Value boolValue = BOOL_VAL(true);
  int   boolIndex = addConstant(&chunk, boolValue);
  TEST_ASSERT_EQUAL_INT_MESSAGE(1, boolIndex, "Second constant should have index 1");
  TEST_ASSERT_EQUAL_MESSAGE(true, AS_BOOL(chunk.constants.values[1]), "Second constant should be true");

  // Test adding nil constant
  Value nilValue = NIL_VAL;
  int   nilIndex = addConstant(&chunk, nilValue);
  TEST_ASSERT_EQUAL_INT_MESSAGE(2, nilIndex, "Third constant should have index 2");
  TEST_ASSERT_TRUE_MESSAGE(IS_NIL(chunk.constants.values[2]), "Third constant should be nil");

  // Test adding duplicate constant
  int dupIndex = addConstant(&chunk, numValue);
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, dupIndex, "Duplicate constant should reuse existing index");
  TEST_ASSERT_EQUAL_INT_MESSAGE(3, chunk.constants.count, "Constants count should remain unchanged");
  TEST_ASSERT_EQUAL_MESSAGE(1.2, AS_NUMBER(chunk.constants.values[0]), "Original constant should be unchanged");

  // Test adding another duplicate constant
  Value boolValue2 = BOOL_VAL(true);
  int   dupIndex2  = addConstant(&chunk, boolValue2);
  TEST_ASSERT_EQUAL_INT_MESSAGE(1, dupIndex2, "Duplicate boolean constant should reuse existing index");
  TEST_ASSERT_EQUAL_INT_MESSAGE(3, chunk.constants.count, "Constants count should remain unchanged after duplicate");
  TEST_ASSERT_EQUAL_MESSAGE(true, AS_BOOL(chunk.constants.values[1]), "Original boolean constant should be unchanged");

  freeChunk(&chunk);
}

void run_chunk_tests(void) {
  RUN_TEST(test_initChunk);
  RUN_TEST(test_writeChunk);
  RUN_TEST(test_freeChunk);
  RUN_TEST(test_addConstant);
}
