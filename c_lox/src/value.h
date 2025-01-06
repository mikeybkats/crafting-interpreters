#ifndef clox_value_h
#define clox_value_h

#include "common.h"

/**
 * ## Struct: Obj
 *
 * @brief The name “Obj” itself refers to a struct that contains the state shared across all object types. It’s sort of
 * like the “base class” for objects. Because of some cyclic dependencies between values and objects, we forward-declare
 * it in the “value” module.
 */
typedef struct Obj Obj;

typedef struct ObjString ObjString;

/**
 * A little bit about values in clox:
 *
 * Values in clox are stored in a constant pool. This is similar to Java --
 * [java constant pool
 * spec](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4)
 *
 * The clox constant pool is an array of values. The instruction to load a data
 * type (like a constant) looks up the value by index in the array.
 */
// typedef double Value;

/**
 * ## Enum: ValueType
 *
 * @brief The type of the value defined in the union type Value
 */
typedef enum {
  VAL_BOOL,
  VAL_NIL,
  VAL_NUMBER,
  VAL_OBJ
} ValueType;

/**
 * ## Struct: Value
 *
 * @brief A value in the constant pool.
 *
 * Defined as a union type to allow for different types of values without wasting memory. A union type lets the data
 * type be used as a single type, but the actual data type is stored in the union.
 */
typedef struct {
  ValueType type;
  union {
    bool   boolean;
    double number;
    Obj*   obj;
  } as;
} Value;

/**
 * ## Macros: Value
 *
 * @brief Macros to check the type of a value
 */
#define IS_BOOL(value)   ((value).type == VAL_BOOL)
#define IS_NIL(value)    ((value).type == VAL_NIL)
#define IS_NUMBER(value) ((value).type == VAL_NUMBER)
#define IS_OBJ(value)    ((value).type == VAL_OBJ)

/**
 * ## Macros: Value
 *
 * @brief Macros to get the value of a value
 */
#define AS_OBJ(value)    ((value).as.obj)
#define AS_BOOL(value)   ((value).as.boolean)
#define AS_NUMBER(value) ((value).as.number)

/**
 * ## Macros: Value
 *
 * @brief Macros to create values with the correct type tag. "This hoists statically typed values up into clox's
 * dynamically typed universe"
 */
#define BOOL_VAL(value)   ((Value){VAL_BOOL, {.boolean = value}})
#define NIL_VAL           ((Value){VAL_NIL, {.number = 0}})
#define NUMBER_VAL(value) ((Value){VAL_NUMBER, {.number = value}})
#define OBJ_VAL(object)   ((Value){VAL_OBJ, {.obj = (Obj*)object}})

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
  int    capacity;
  int    count;
  Value* values;
} ValueArray;

bool valuesEqual(Value a, Value b);
void initValueArray(ValueArray* array);
void writeValueArray(ValueArray* array, Value value);
void freeValueArray(ValueArray* array);
void printValue(Value value);

#endif