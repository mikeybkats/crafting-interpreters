#ifndef clox_value_h
#define clox_value_h

#include "common.h"

typedef double Value;

/*
 * ## Struct: ValueArray
 *
 * @brief A dynamic array of Values
 *
 * @param capacity (int) the number of elements in the array that have been
 * allocated
 * @param count (int) how many entries in the array are in use
 * @param values (Value*) the pointer to the array of values
 */
typedef struct {
  int capacity;
  int count;
  Value* values;
} ValueArray;

void initValueArray(ValueArray* array);
void writeValueArray(ValueArray* array, Value value);
void freeValueArray(ValueArray* array);
void printValue(Value value);

#endif