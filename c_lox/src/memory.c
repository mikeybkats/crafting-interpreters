#include "memory.h"

#include <stdlib.h>

/*
 *
 * ## reallocate
 * ### used for all dynamic memory allocation in clox
 *
 * "allocating memory, freeing it, and changing the size of an existing
 * allocation. Routing all of those operations through a single function will be
 * important later when we add a garbage collector that needs to keep track of
 * how much memory is in use."
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

  void* result = realloc(pointer, newSize);
  if (result == NULL) exit(1);
  return result;
};