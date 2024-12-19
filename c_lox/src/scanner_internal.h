#ifndef clox_scanner_internal_h
#define clox_scanner_internal_h

#include "scanner.h"

// Expose scanner state for testing
extern Scanner scanner;

#ifdef DEBUG_TEST
// Add test helper functions
const char* test_get_scanner_current(void);
const char* test_get_scanner_start(void);
int test_get_scanner_line(void);
#endif

#endif