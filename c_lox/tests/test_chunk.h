#ifndef CLOX_TEST_CHUNK_H
#define CLOX_TEST_CHUNK_H

void setUpChunk(void);
void tearDownChunk(void);

void test_initChunk(void);
void test_writeChunk(void);
void test_freeChunk(void);

// Run all chunk tests
void run_chunk_tests(void);

#endif  // CLOX_TEST_CHUNK_H