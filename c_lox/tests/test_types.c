#include "test_types.h"

#include <string.h>

#include "../src/object.h"
#include "../src/value.h"
#include "unity/src/unity.h"

void test_string_value_creation(void) {
  const char* input = "hello world";

  ObjString* string = copyString(input, strlen(input));

  char* test_string        = string->chars;
  int   test_string_length = string->length;
  int   test_string_type   = string->obj.type;

  TEST_ASSERT_EQUAL_STRING("hello world", test_string);
  TEST_ASSERT_EQUAL_INT(11, test_string_length);
  TEST_ASSERT_EQUAL_INT(OBJ_STRING, test_string_type);
}

void run_types_tests(void) {
  RUN_TEST(test_string_value_creation);
}