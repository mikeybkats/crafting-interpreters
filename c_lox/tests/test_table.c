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

void test_table_find_entry(void) {
  Value key   = NUMBER_VAL(3);
  Value value = NUMBER_VAL(2);

  tableSet(&table, &key, &value);

  Entry* entry = findEntry(table.entries, table.capacity, &key);

  TEST_ASSERT_EQUAL_INT(AS_NUMBER(entry->key), AS_NUMBER(key));
  TEST_ASSERT_EQUAL_INT(AS_NUMBER(entry->value), AS_NUMBER(value));
}

void test_table_set(void) {
  Value key   = NUMBER_VAL(1);
  Value value = NUMBER_VAL(2);

  bool newKey = tableSet(&table, &key, &value);
  int  index  = getEntryIndex(&table, &key);

  TEST_ASSERT_EQUAL_INT(table.count, 1);
  TEST_ASSERT_EQUAL_INT(table.capacity, 8);
  TEST_ASSERT_EQUAL_INT(newKey, true);
  TEST_ASSERT_EQUAL_INT(AS_NUMBER(table.entries[index].key), AS_NUMBER(key));
  TEST_ASSERT_EQUAL_INT(AS_NUMBER(table.entries[index].value), AS_NUMBER(value));
}

void test_table_get(void) {
  Value key   = NUMBER_VAL(3);
  Value value = NUMBER_VAL(4);

  tableSet(&table, &key, &value);

  Value result = NIL_VAL;
  tableGet(&table, &key, &result);

  TEST_ASSERT_EQUAL_INT(AS_NUMBER(result), AS_NUMBER(value));
}

void test_table_delete(void) {
  Value key   = NUMBER_VAL(5);
  Value value = NUMBER_VAL(6);

  tableSet(&table, &key, &value);
  tableDelete(&table, &key);

  int  index     = getEntryIndex(&table, &key);
  bool tombstone = isTombstone(&table.entries[index]);

  TEST_ASSERT_EQUAL_INT(tombstone, true);
}

void run_table_tests(void) {
  test_table_setup();
  RUN_TEST(test_table_init);
  RUN_TEST(test_table_find_entry);
  RUN_TEST(test_table_set);
  RUN_TEST(test_table_get);
  RUN_TEST(test_table_delete);
  test_table_teardown();
}