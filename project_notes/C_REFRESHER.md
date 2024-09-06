# C Refresher

## TOC

- [C Refresher](#c-refresher)
  - [TOC](#toc)
  - [Quiz](#quiz)
  - [Common Lang Features:](#common-lang-features)
  - [Makefiles](#makefiles)
  - [Libraries](#libraries)
    - [stdlib.h](#stdlibh)

## Quiz

Q: _When creating a variable in the below context, what does the `&` symbol do? Why is it important?_

```c
int main() {
   CustomType foo;
   initFoo(&foo);
   printFoo(&foo);
}
```

A: the `&` symbol creates a pointer to the address of foo - its called the "address-of" operator. Foo can be accessed like this:

```c
CustomType *fooPointer = &foo;  // Declare a pointer to CustomType and initialize it with the address of foo
foo.someField = 10;        // Access foo directly
(*fooPointer).someField = 10;  // Access foo through the pointer (dereferencing)
fooPointer->someField = 10;
```

Q: _What are the practical differences between assigning directly and assigning via pointer?_

A:

- Direct assignment `foo.someField = 10;` is typically happens on the stack and with static and global variables - the compiler knows where the variable resides. In order to to this the compiler must see the object in the same scope as the assignment.
- Dereferenced pointer assignment `foo->someField = 10;` accesses an object indirectly by memory address. This means the object could exist on the stack, heap or elsewhere. And, assignement can happen anywhere.

Importance:

- It allows functions like initFoo and printFoo to access the original variable
- This is more memory efficient than copying an entire object to access data

Q: _What smells with the below code sample?_

```c
void initFoo(CustomType foo);
void printFoo(CustomType foo);
```

A: The functions take custom types without pointers. So, the CustomType object is being copied and passed into the fuction. This is memory ineficient and also will not modify the original object.

Q: _What does `#ifndef` and `#endif` do?_

A: `#ifndef` stands for _if not defined_. It's saying to define something if it does not exist. It relates directly to headers:

```c
// myheader.h

#ifndef MYHEADER_H  // Check if MYHEADER_H is not defined
#define MYHEADER_H  // Define MYHEADER_H

// Declarations and definitions here
void myFunction();

#endif // End of the conditional directive

```

Q: _When using `#ifndef` in the way shown above what's it called?_

A: Header Guards. Header Guards are used to prevent multiple inclusions of the same file.

Q: _How do `.c` files and `.h` files differ?_

A:

- `.c` files are "source files." They include the actual code that defines the program's logic (functions, variables, conditionals, ect). `.c` files compile into `.o` object files by the compiler that are then linked and executed.
- `.h` files are "header files." They are used to make declarations (functions, macros, constants) that `.c` files reference and use. Specifically `.h` files have content not found in `.c` files:
  - `typedef` - declares new data types and structs
  - `#define` - defines macros or constants
  - function prototypes - defines function signatures without providing logic
  - Include guards - `#ifndef` `#endif`

Q: _How does the compiler treat `.c` and `.h` files?_

A: `.h` files are not compiled. They are included as part of pre-compilation. When a `.c` file uses `#include` to reference a `.h` file, it's actually telling the pre-compiler to copy and paste the contents of the `.h` file directly into the `.c` file. The output of the pre-compiler gets passed to the compiler for final processing.

Q: _What's bad about the singleton pattern?_

A:

## Common Lang Features:

`*` - Dereference operator - dereferences a pointer;

```
int var = 10;
int *ptr = &var;
int valueAtPtr = *ptr; // valueAtPtr is now 10, which is the value of var
```

`&` - Address of operator - creates a pointer to the data. gets the value of the address.

The & symbol is crucial here for efficient memory usage and allowing direct manipulation of the foo variable by the called functions.

```
CustomType foo;
initFoo(&foo);
printFoo(&foo);

CustomType *fooPointer = &foo; // fooPointer now holds a pointer to the address of the foo variable.
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

## Libraries

### stdlib.h

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
