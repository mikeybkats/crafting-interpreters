#include "test_compiler.h"

#include "../src/chunk.h"
#include "../src/compiler.h"
#include "../src/scanner.h"
#include "unity/src/unity.h"

// Global test variables matching compiler.c globals
static Parser parser;          // matches: Parser parser
static Chunk* compilingChunk;  // matches: Chunk* compilingChunk

// Rules array is also global in compiler.c but it's const, so we don't need to test it directly
// ParseRule rules[] = { ... }

void setUpCompiler(void) {
  // Initialize globals before each test
  compilingChunk   = NULL;
  parser.hadError  = false;
  parser.panicMode = false;
}

void tearDownCompiler(void) {
  // Clean up after each test
  if (compilingChunk != NULL) {
    freeChunk(compilingChunk);
    compilingChunk = NULL;
  }
}

// Core compilation tests
void test_compile(void) {
  initChunk(compilingChunk);
  const bool result = compile("1 + 2 * 3", compilingChunk);
  TEST_ASSERT_TRUE_MESSAGE(result, "Compilation should succeed");
  TEST_ASSERT_EQUAL_INT_MESSAGE(compilingChunk->count, 3, "Chunk should have 3 instructions");
  TEST_ASSERT_EQUAL_INT_MESSAGE(compilingChunk->constants.count, 0, "Chunk should have 0 constants");
  freeChunk(compilingChunk);
}

void test_compile_error_handling(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

// Parser tests
void test_parser_advance(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_parser_consume(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_parser_error_handling(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

// Chunk emission tests
void test_emit_byte(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_emit_bytes(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_emit_return(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_emit_constant(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_make_constant(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

// Expression parsing tests
void test_number(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_grouping(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_unary(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_binary(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_expression(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

// Precedence parsing tests
void test_parse_precedence(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_get_rule(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_current_chunk(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

// Error reporting tests
void test_error_at(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_error(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void test_error_at_current(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

// End compiler tests
void test_end_compiler(void) { TEST_FAIL_MESSAGE("Test not implemented"); }

void run_compiler_tests(void) {
  printf("\n\033[0;31mTest compiler ---\033[0m \n");
  // Core compilation tests
  RUN_TEST(test_compile);
  RUN_TEST(test_compile_error_handling);

  // Parser tests
  RUN_TEST(test_parser_advance);
  RUN_TEST(test_parser_consume);
  RUN_TEST(test_parser_error_handling);

  // Chunk emission tests
  RUN_TEST(test_emit_byte);
  RUN_TEST(test_emit_bytes);
  RUN_TEST(test_emit_return);
  RUN_TEST(test_emit_constant);
  RUN_TEST(test_make_constant);

  // Expression parsing tests
  RUN_TEST(test_number);
  RUN_TEST(test_grouping);
  RUN_TEST(test_unary);
  RUN_TEST(test_binary);
  RUN_TEST(test_expression);

  // Precedence parsing tests
  RUN_TEST(test_parse_precedence);
  RUN_TEST(test_get_rule);
  RUN_TEST(test_current_chunk);

  // Error reporting tests
  RUN_TEST(test_error_at);
  RUN_TEST(test_error);
  RUN_TEST(test_error_at_current);

  // End compiler tests
  RUN_TEST(test_end_compiler);
}
