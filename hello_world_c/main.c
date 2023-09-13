#include <CUnit/Basic.h>
#include <CUnit/CUnit.h>
#include <stdio.h>
#include <stdlib.h>

#include "src/ll_nodes.h"

// Your test functions go here
void test_create_heap_string() {
  char *sourceString = "Hello, World!";
  char *result = create_heap_string(sourceString);
  CU_ASSERT_PTR_NOT_NULL(result);
  CU_ASSERT_STRING_EQUAL(result, sourceString);
  free(result);
}

int init_suite(void) { return 0; }

int clean_suite(void) { return 0; }

int main() {
  CU_pSuite suite = NULL;

  // Initialize the CUnit test registry
  if (CUE_SUCCESS != CU_initialize_registry()) {
    return CU_get_error();
  }

  // Create a test suite
  suite = CU_add_suite("MySuite", init_suite, clean_suite);
  if (NULL == suite) {
    CU_cleanup_registry();
    return CU_get_error();
  }

  // Add your test functions to the suite
  if (NULL ==
      CU_add_test(suite, "Test create_heap_string", test_create_heap_string)) {
    CU_cleanup_registry();
    return CU_get_error();
  }

  // Run tests using the basic interface
  CU_basic_set_mode(CU_BRM_VERBOSE);
  CU_basic_run_tests();
  CU_cleanup_registry();
  return CU_get_error();
}
