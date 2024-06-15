## Common Lang Features:

`*` - Dereference operator - dereferences a pointer;

```
int var = 10;
int *ptr = &var;
int valueAtPtr = *ptr; // valueAtPtr is now 10, which is the value of var
```

`&` - Address of operator - gets the value of the address.

```
int var = 10;
int *ptr = &var; // ptr now holds the address of var
```

## Makefiles

consider the pattern in a make file

```
src/%.o: src/%.c
	clang -I. -std=c11 -Wall -c $< -o $@
```

`src/%.o: src/%.c` is a rule

- `src/%.o` is the target pattern
- `src/%.c` is the prerequisite pattern - this represents the files to be selected
- `$<` is a special variable in Makefiles that represents the first prerequisite of the rule
- `$@` is a special variable that represents the target of the rule `src/%.o`.
- `$^` represents all the prerequisites of the target, excluding duplicates

so the final output in the terminal will be:

`clang -I -std=c11 -Wall -c src/%.c -o src/%.o`

## stdlib.h

The C standard library (`stdlib.h`) provides several memory-related functions for dynamic memory allocation, memory management, and conversion. Here are the most commonly used memory functions offered by `stdlib.h`, along with their brief descriptions:

1. **Memory Allocation and Deallocation**:

   - `malloc`: Allocates a block of memory of a specified size and returns a pointer to the beginning of the allocated memory.
   - `calloc`: Allocates a block of memory for an array of elements, initializes all bits to zero, and returns a pointer to the allocated memory.
   - `realloc`: Changes the size of the previously allocated memory block, possibly moving it to a different location in memory, and returns a pointer to the resized memory block.
   - `free`: Deallocates the previously allocated memory block, making it available for subsequent reallocation.

2. **Memory Utilities**:

   - `memcpy`: Copies a block of memory from one location to another, allowing overlapping memory regions.
   - `memmove`: Similar to `memcpy`, but guarantees correct behavior even if the source and destination memory regions overlap.
   - `memcmp`: Compares two blocks of memory and returns an integer representing their comparison result.
   - `memset`: Sets a block of memory to a specified value (usually a byte value).

3. **String Conversion**:
   - `atoi`: Converts a string representing an integer into an integer value.
   - `atol`: Converts a string representing a long integer into a long integer value.
   - `atoll`: Converts a string representing a long long integer into a long long integer value.
   - `atof`: Converts a string representing a floating-point number into a double value.
   - `strtol`: Converts a string to a long integer value, allowing more control over base and error detection.
   - `strtoll`: Converts a string to a long long integer value, allowing more control over base and error detection.
   - `strtoul`: Converts a string to an unsigned long integer value, allowing more control over base and error detection.
   - `strtoull`: Converts a string to an unsigned long long integer value, allowing more control over base and error detection.
   - `strtof`: Converts a string to a float value, allowing more control over error detection.
   - `strtod`: Converts a string to a double value, allowing more control over error detection.
   - `strtold`: Converts a string to a long double value, allowing more control over error detection.

These functions provide essential capabilities for memory allocation, manipulation, and conversion in C programming. Understanding and properly using these functions is crucial for writing efficient and robust C code.
