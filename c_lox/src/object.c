#include "object.h"

#include <stdio.h>
#include <string.h>

#include "memory.h"
#include "table.h"
#include "value.h"
#include "vm.h"

/**
 * ## Macro: ALLOCATE_OBJ
 *
 * @brief Type safe macro that takes a specific type, handles size calculation, casts the result to the correct type
 */
#define ALLOCATE_OBJ(type, objectType) (type*)allocateObject(sizeof(type), objectType)

/**
 * ## Function: allocateObject
 *
 * @brief Allocates raw memory, sets the object type, returns generic Obj*
 */
static Obj* allocateObject(size_t size, ObjType type) {
  Obj* object  = (Obj*)reallocate(NULL, 0, size);
  object->type = type;

  object->next = vm.objects;
  vm.objects   = object;
  return object;
}

static ObjString* allocateString(char* chars, int length, uint32_t hash) {
  ObjString* string = ALLOCATE_OBJ(ObjString, OBJ_STRING);
  Value*     value  = ALLOCATE_OBJ(Value, OBJ_NIL);
  string->length    = length;
  string->chars     = chars;
  string->hash      = hash;

  Value* stringValue  = ALLOCATE_OBJ(Value, OBJ_STRING);
  stringValue->type   = VAL_OBJ;
  stringValue->as.obj = (Obj*)string;

  tableSet(&vm.strings, stringValue, *value);
  return string;
}

static uint32_t hashString(const char* key, int length) {
  uint32_t hash = 2166136261u;
  for (int i = 0; i < length; i++) {
    hash ^= (uint8_t)
        key[i];  // ^= is the XOR assignment operator. performs bitwise XOR operation between hash and character value
    hash *= 16777619;
  }

  return hash;
}

/**
 * ## Function: takeString
 *
 * @brief Takes ownership of the string, so the caller must not free it.
 */
ObjString* takeString(char* chars, int length) {
  uint32_t hash = hashString(chars, length);

  // check if there is an interned string
  ObjString* interned = tableFindString(&vm.strings, chars, length, hash);
  // if there is an interned string
  if (interned != NULL) {
    // free the string and return the interned string
    FREE_ARRAY(char, chars, length + 1);
    return interned;
  }

  return allocateString(chars, length, hash);
}

ObjString* copyString(const char* chars, int length) {
  uint32_t hash = hashString(chars, length);

  // the string only gets added if it's unique. so check to see if it exists in the interned Table of strings
  ObjString* interned = tableFindString(&vm.strings, chars, length, hash);
  // if it does exist then return the interned string instead
  if (interned != NULL) return interned;

  char* heapChars = ALLOCATE(char, length + 1);
  memcpy(heapChars, chars, length);
  heapChars[length] = '\0';

  return allocateString(heapChars, length, hash);
}

void printObject(Value value) {
  switch (OBJ_TYPE(value)) {
    case OBJ_BOOL:
      printf(AS_BOOL(value) ? "true" : "false");
      break;
    case OBJ_NIL:
      printf("nil");
      break;
    case OBJ_NUMBER:
      printf("%g", AS_NUMBER(value));
      break;
    case OBJ_STRING:
      printf("%s", AS_CSTRING(value));
      break;
    default:
      printf("<object>");
      break;
  }
}
