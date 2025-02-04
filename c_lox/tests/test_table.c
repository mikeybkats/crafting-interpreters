#include "test_table.h"

#include <stdio.h>
#include <string.h>

#include "../src/memory.h"
#include "../src/object.h"
#include "../src/table.h"
#include "../src/value.h"
#include "unity/src/unity.h"

Table table;

void test_table_setup(void) {
  initTable(&table);
}

void test_table_teardown(void) {
  freeTable(&table);
}

void test_table_init(void) {
  TEST_ASSERT_EQUAL_INT(table.count, 0);
  TEST_ASSERT_EQUAL_INT(table.capacity, 0);
  TEST_ASSERT_NULL(table.entries);
}

void test_table_set(void) {
  Value key   = NUMBER_VAL(1);
  Value value = NUMBER_VAL(2);

  tableSet(&table, &key, &value);

  TEST_ASSERT_EQUAL_INT(table.count, 1);
  TEST_ASSERT_EQUAL_INT(table.capacity, 8);
  TEST_ASSERT_EQUAL_INT(table.entries[0].key.as.number, key.as.number);
  TEST_ASSERT_EQUAL_INT(table.entries[0].value.as.number, value.as.number);
}

void run_table_tests(void) {
  test_table_setup();
  RUN_TEST(test_table_init);
  RUN_TEST(test_table_set);
  test_table_teardown();
}