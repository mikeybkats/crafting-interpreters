#include "test_table.h"

#include <stdio.h>
#include <string.h>

#include "../src/memory.h"
#include "../src/object.h"
#include "../src/table.h"
#include "../src/value.h"
#include "unity/src/unity.h"

void test_table_setup(void) {
}

void test_table_teardown(void) {
}

void test_table_init(void) {
}

void run_table_tests(void) {
  test_table_setup();
  RUN_TEST(test_table_init);
  test_table_teardown();
}