#include "value.h"

#include <stdio.h>
#include <string.h>

#include "memory.h"
#include "object.h"

void initValueArray(ValueArray* array) {
  array->capacity = 0;
  array->count    = 0;
  array->values   = NULL;
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
    array->values   = GROW_ARRAY(Value, array->values, oldCapacity, array->capacity);
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

void printValue(Value value) {
  switch (value.type) {
    case VAL_BOOL:
      printf(AS_BOOL(value) ? "true" : "false");
      break;
    case VAL_NIL:
      printf("nil");
      break;
    case VAL_NUMBER:
      printf("%g", AS_NUMBER(value));
      break;
    case VAL_OBJ:
      printObject(value);
      break;
  }
}

bool valuesEqual(Value a, Value b) {
  if (a.type != b.type) return false;

  switch (a.type) {
    case VAL_BOOL:
      return AS_BOOL(a) == AS_BOOL(b);
    case VAL_NIL:
      return true;
    case VAL_NUMBER:
      return AS_NUMBER(a) == AS_NUMBER(b);
    case VAL_OBJ:
      // expands to pointers to a.as.obj == b.as.obj;
      // !! this compares the interned string objects not the actual values
      return AS_OBJ(a) == AS_OBJ(b);  // compares the memory address of the objects not the contents or actual values
    default:
      return false;
  }
}
