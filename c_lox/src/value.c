#include "value.h"

#include <stdio.h>

#include "memory.h"

void initValueArray(ValueArray* array) {
  array->capacity = 0;
  array->count = 0;
  array->values = NULL;
}

/*
 * ## Function: writeValueArray
 *
 * @brief dynamically Appends a Value to a ValueArray, growing the array if more
 * size is needed.
 *
 * @param array the ValueArray to append the value to
 * @param value the Value to append to the array
 */
void writeValueArray(ValueArray* array, Value value) {
  if (array->capacity < array->count + 1) {
    int oldCapacity = array->capacity;
    array->capacity = GROW_CAPACITY(oldCapacity);
    array->values =
        GROW_ARRAY(Value, array->values, oldCapacity, array->capacity);
  }

  array->values[array->count] = value;  // append value to values
  array->count++;                       // increment count
}

/*
 * ## Function: freeValueArray
 *
 * @brief Frees a ValueArray from memory.
 *
 * @param array the ValueArray to clear.
 */
void freeValueArray(ValueArray* array) {
  FREE_ARRAY(Value, array->values, array->capacity);
  initValueArray(array);
}

void printValue(Value value) { printf("%g", value); }