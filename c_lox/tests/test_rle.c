#include "../src/rle.h"
#include "unity/src/unity.h"

void test_rleEncode(void) {
  printf("Test rleEncode --- ");
  int data[] = {123, 123, 123, 0, 50, 50, 110, 110};

  RleData* encodedArr = rleEncode(data, 8);

  // printf("\n == Result ==\n");
  // for (int i = 0; i < 8; i++) {
  //   if (encodedArr->encodedData[i] != NULL) {
  //     printf("%s \n", encodedArr->encodedData[i]);
  //   }
  // }

  TEST_ASSERT_EQUAL_STRING_MESSAGE("123 x 3", encodedArr->encodedData[0],
                                   "Should be '123 x 3'");
  TEST_ASSERT_EQUAL_STRING_MESSAGE("0 x 1", encodedArr->encodedData[1],
                                   "Should be '0 x 1'");
  TEST_ASSERT_EQUAL_STRING_MESSAGE("50 x 2", encodedArr->encodedData[2],
                                   "Should be '50 x 2'");
  TEST_ASSERT_EQUAL_STRING_MESSAGE("110 x 2", encodedArr->encodedData[3],
                                   "Should be '110 x 2'");
}