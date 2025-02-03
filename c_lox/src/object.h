#ifndef clox_object_h
#define clox_object_h

#include "common.h"
#include "value.h"

#define OBJ_TYPE(value) (AS_OBJ(value)->type)

#define IS_STRING(value) isObjType(value, OBJ_STRING)

/*
From the C standard:

### § 6.7.2.1 13

Within a structure object, the non-bit-field members and the units in which bit-fields reside have addresses that
increase in the order in which they are declared. A pointer to a structure object, suitably converted, points to its
initial member (or if that member is a bit-field, then to the unit in which it resides), and vice versa. There may be
unnamed padding within a structure object, but not at its beginning.
*/

#define AS_STRING(value) ((ObjString*)AS_OBJ(value))

/**
 * ## Macro: AS_CSTRING
 *
 * @brief gets the actual string of chars
 */
#define AS_CSTRING(value) (((ObjString*)AS_OBJ(value))->chars)

/**
 * ## Enum: ObjType
 *
 * @brief Enum for the types of objects.
 */
typedef enum
{
  OBJ_STRING,
} ObjType;

/**
 * ## Struct: Obj
 *
 * @brief Base struct for all objects.
 */
struct Obj
{
  ObjType     type;
  struct Obj* next;
};

/**
 * ## Struct: ObjString
 *
 * @brief A string object is a struct that contains a pointer to the Obj base struct, the length of the string, and a
 * pointer to the characters in the string.
 *
 * @note ObjString can safely be ast to Obj because its first member is of type Obj.
 */
struct ObjString
{
  /* First 'slot' is the Obj base struct */
  Obj obj;     /*
      {
        ObjType Type;
        struct Obj* next;
      }
               */
  int length;  // store the length, which indicates the number of bytes in the array and allows for easier access to the
               // null terminator.
  char*    chars;
  uint32_t hash;  // "calculate the hash code once up front and be certain that it will never get invalidated"
};

ObjString* copyString(const char* chars, int length);
ObjString* takeString(char* chars, int length);

void printObject(Value value);

static inline bool isObjType(Value value, ObjType type) {
  return IS_OBJ(value) && AS_OBJ(value)->type == type;
}

#endif
