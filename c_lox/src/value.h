#ifndef clox_value_h
#define clox_value_h

#include "common.h"

/**
 * ## Enum: ValueType
 *
 * A little bit about values in clox:
 *
 * Values in clox are stored in a constant pool. This is similar to Java --
 * [java constant pool
 * spec](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4)
 *
 * The clox constant pool is an array of values. The instruction to load a data
 * type (like a constant) looks up the value by index in the array.
 */
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