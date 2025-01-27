#include "object.h"

#include <stdio.h>
#include <string.h>

#include "memory.h"
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

static ObjString* allocateString(char* chars, int length) {
  ObjString* string = ALLOCATE_OBJ(ObjString, OBJ_STRING);
  string->length    = length;
  string->chars     = chars;
  return string;
}

/**
 * ## Function: takeString
 *
 * @brief Takes ownership of the string, so the caller must not free it.
 */
ObjString* takeString(char* chars, int length) {
  return allocateString(chars, length);
}

ObjString* copyString(const char* chars, int length) {
  char* heapChars = ALLOCATE(char, length + 1);
  memcpy(heapChars, chars, length);
  heapChars[length] = '\0';
  return allocateString(heapChars, length);
}

void printObject(Value value) {
  switch (OBJ_TYPE(value)) {
    case OBJ_STRING:
      printf("%s", AS_CSTRING(value));
      break;
  }
}
