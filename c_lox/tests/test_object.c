#include "test_object.h"

#include <stdlib.h>
#include <string.h>

#include "../src/object.h"
#include "../src/value.h"
#include "../src/vm.h"
#include "unity/src/unity.h"

VM vm;

void setup_test_object(void) {
  vm.objects = NULL;
  initTable(&vm.strings);
}

void teardown_test_object(void) {
  freeTable(&vm.strings);
  Obj* object = vm.objects;
  while (object != NULL) {
    Obj* next = object->next;
    free(object);
    object = next;
  }
}

void test_string_value_creation(void) {
  const char* input = "hello world";

  ObjString* string = copyString(input, strlen(input));

  char* test_string        = string->chars;
  int   test_string_length = string->length;
  int   test_string_type   = string->obj.type;

  TEST_ASSERT_EQUAL_STRING("hello world", test_string);
  TEST_ASSERT_EQUAL_INT(11, test_string_length);
  TEST_ASSERT_EQUAL_INT(OBJ_STRING, test_string_type);

  Value stringValue = OBJ_VAL(string);

  TEST_ASSERT_EQUAL_INT(VAL_OBJ, stringValue.type);
}

void test_number_value_creation(void) {
  double value       = 1.2;
  Value  numberValue = NUMBER_VAL(value);

  TEST_ASSERT_EQUAL_INT(VAL_NUMBER, numberValue.type);
  TEST_ASSERT_EQUAL_FLOAT(value, AS_NUMBER(numberValue));
}

void test_string_interning_pointer_equality(void) {
  ObjString* string1 = copyString("hello", 5);
  ObjString* string2 = copyString("hello", 5);

  ObjString* string3 = copyString("world", 5);
  ObjString* string4 = copyString("wOrlD", 5);

  Value hello1 = OBJ_VAL(string1);
  Value hello2 = OBJ_VAL(string2);

  Value world1 = OBJ_VAL(string3);
  Value world2 = OBJ_VAL(string4);

  TEST_ASSERT_EQUAL_INT(AS_OBJ(hello1), AS_OBJ(hello2));
  TEST_ASSERT_NOT_EQUAL_INT(AS_OBJ(world1), AS_OBJ(world2));
}

void run_object_tests(void) {
  setup_test_object();
  RUN_TEST(test_string_value_creation);
  RUN_TEST(test_number_value_creation);
  RUN_TEST(test_string_interning_pointer_equality);
  teardown_test_object();
}