#ifndef clox_memory_h
#define clox_memory_h

#include "common.h"

/*
 * ## Macro: GROW_CAPACITY
 *
 * @brief given an integer (size in bits) it returns twice the value. If the
 * value is less than 8 (a single byte) it will return the given value.
 *
 * @param capacity the size int to grow
 */
#define GROW_CAPACITY(capacity) ((capacity) < 8 ? 8 : (capacity) * 2)

/*
 * ## Macro: GROW_ARRAY
 *
 * @brief Grows the size of an array from the oldCount to the new count.
 *
 * @param type the type of the value of values in the array
 * @param pointer a pointer to the block in memory of the array
 * @param oldCount old length of block
 * @param newCount new length of block
 */
#define GROW_ARRAY(type, pointer, oldCount, newCount)   \
  (type*)reallocate(pointer, sizeof(type) * (oldCount), \
                    sizeof(type) * (newCount))

/*
 * ## Macro: FREE_ARRAY

 * @brief Frees the array from memory.
 *
 * @param type the data type (value of values) inside the array that is to be
 * freed.
 *
 * @param pointer a pointer to the array itself
 *
 * @param oldCount the capacity of the array to be freed from memory
 */
#define FREE_ARRAY(type, pointer, oldCount) \
  (type*)reallocate(pointer, sizeof(type) * (oldCount), 0)

void* reallocate(void* pointer, size_t oldSize, size_t newSize);

#endif