#include "../src/rle.h"
#include "unity/src/unity.h"

void test_rleEncodeLines(void) {
  printf("Test rleEncode --- ");
  int data[] = {123, 123, 123, 0, 50, 50, 110, 110};

  RleData* encodedArr = rleEncodeLines(data, 8);

  // printf("\n == Result ==\n");
  // for (int i = 0; i < 8; i++) {
  //   if (encodedArr->encodedData[i] != NULL) {
  //     printf("%s \n", encodedArr->encodedData[i]);
  //   }
  // }

  TEST_ASSERT_EQUAL_STRING_MESSAGE(
      "123 x 3, 0 x 1, 50 x 2, 110 x 2", encodedArr->encodedData,
      "Should be '123 x 3, 0 x 1, 50 x 2, 110 x 2'");
}

void test_rleDecodeLines(void) {
  printf("Test rledecode --- \n");
  int data[] = {123, 123, 123, 0, 50, 50, 110, 110};

  RleData* encodedArr = rleEncodeLines(data, 8);

  int* decodedData = rleDecodeLines(encodedArr);

  printf("\n == Result ==\n");
  for (int i = 0; i < 8; i++) {
    if (decodedData[i] != NULL) {
      printf("%d \n", decodedData[i]);
    }
  }

  TEST_ASSERT_EQUAL_UINT32_ARRAY(
      data, decodedData, "Should be '{123, 123, 123, 0, 50, 50, 110, 110}'");
}