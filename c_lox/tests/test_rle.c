#include "test_rle.h"

#include "../src/rle.h"
#include "unity/src/unity.h"

void test_rleEncodeLines(void) {
  printf("\n\033[0;31mTest rleEncode ---\033[0m \n");
  int data[] = {123, 123, 123, 0, 50, 50, 110, 110};

  RleData* encodedArr = rleEncodeLines(data, 8);

  TEST_ASSERT_EQUAL_STRING_MESSAGE("123 x 3, 0 x 1, 50 x 2, 110 x 2", encodedArr->encodedData,
                                   "Should be '123 x 3, 0 x 1, 50 x 2, 110 x 2'");
}

void test_rleDecodeLines(void) {
  printf("\n\033[0;31mTest rleDecode ---\033[0m \n");
  int data[] = {123, 123, 123, 0, 50, 50, 110, 110};

  RleData* encodedArr = rleEncodeLines(data, 8);

  int decodedLength = 0;
  int* decodedData  = rleDecodeLines(encodedArr, &decodedLength);

  TEST_ASSERT_EQUAL_UINT_ARRAY(data, decodedData, decodedLength);
}

void test_getLine(void) {
  printf("\n\033[0;31mTest getLine ---\033[0m \n");
  int data[] = {123, 123, 123, 0, 50, 50, 110, 110};

  RleData* encodedArr = rleEncodeLines(data, 8);

  TEST_ASSERT_EQUAL_INT_MESSAGE(123, getLine(encodedArr, 0), "Should be 123");
  TEST_ASSERT_EQUAL_INT_MESSAGE(0, getLine(encodedArr, 3), "Should be 0");
  TEST_ASSERT_EQUAL_INT_MESSAGE(50, getLine(encodedArr, 4), "Should be 50");
  TEST_ASSERT_EQUAL_INT_MESSAGE(110, getLine(encodedArr, 6), "Should be 110");
}

void run_rle_tests(void) {
  RUN_TEST(test_rleEncodeLines);
  RUN_TEST(test_rleDecodeLines);
  RUN_TEST(test_getLine);
}