#ifndef CLOX_TEST_COMPILER_H
#define CLOX_TEST_COMPILER_H

void setUpCompiler(void);
void tearDownCompiler(void);

// Core compilation tests
void test_compile(void);
void test_compile_error_handling(void);

// Parser tests
void test_parser_advance(void);
void test_parser_consume(void);
void test_parser_error_handling(void);

// Chunk emission tests
void test_emit_byte(void);
void test_emit_bytes(void);
void test_emit_return(void);
void test_emit_constant(void);
void test_make_constant(void);

// Expression parsing tests
void test_number(void);
void test_grouping(void);
void test_unary(void);
void test_binary(void);
void test_expression(void);

// Precedence parsing tests
void test_parse_precedence(void);
void test_get_rule(void);
void test_current_chunk(void);

// Error reporting tests
void test_error_at(void);
void test_error(void);
void test_error_at_current(void);

// End compiler tests
void test_end_compiler(void);

// Run all compiler tests
void run_compiler_tests(void);

#endif  // CLOX_TEST_COMPILER_H
