#include "memory.h"

#include <stdlib.h>

#include "vm.h"

/*
 *
 * ## reallocate
 *
 * used for all dynamic memory allocation in clox
 *
 * "allocating memory, freeing it, and changing the size of an existing
 * allocation. Routing all of those operations through a single function will be
 * important later when we add a garbage collector that needs to keep track of
 * how much memory is in use."
 *
 * pointer the pointer to the block in memory
 * oldSize the size of the block in memory
 *
 * ### reallocate handles allocations like this:
 *
 * | oldSize	| newSize               |      Operation                |
 * | ---------- | --------------------- | ----------------------------- |
 * | 0	        | Non‑zero	            | Allocate new block.           |
 * | Non‑zero	| 0	                    | Free allocation               |
 * | Non‑zero	| Smaller than oldSize	| Shrink existing allocation    |
 * | Non‑zero	| Larger than oldSize	| Grow existing allocation.     |
 *
 */
void* reallocate(void* pointer, size_t oldsize, size_t newSize) {
  if (newSize == 0) {
    free(pointer);
    return NULL;
  }

  void* result = realloc(pointer, newSize);  // change the size of the block in memory
                                             // without losing the pointer data.
  if (result == NULL) exit(1);
  return result;
};

static void freeObject(Obj* object) {
  switch (object->type) {
    case OBJ_STRING: {
      ObjString* string = (ObjString*)object;
      FREE_ARRAY(char, string->chars, string->length + 1);
      FREE(ObjString, object);
      break;
    }
  }
}

void freeObjects() {
  Obj* object = vm.objects;
  while (object != NULL) {
    Obj* next = object->next;
    free(object);
    object = next;
  }
}