# C Refresher

## TOC

- [C Refresher](#c-refresher)
  - [TOC](#toc)
  - [Quiz](#quiz)
  - [Common Lang Features:](#common-lang-features)
    - [pointers](#pointers)
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

Q: _What's this do? And, why?_

```c
for (;;) {
    // Loop body
}
```

A: It's an Infinite loop. The reason it's an infinite loop is becuase it's passing undefined to all three expressions. There is no counter, there is not condition, there is no incrementer.

Q: _What is unspecified evaluation order in C? Why does it exist?_

A: There is no guarentee with the order of operations. For example:

```c
int i = 0;
int array[] = {1, 2, 3, 4, 5};
int result = array[i] + (i = 2);  // Unspecified behavior
// The result could be either 3 (1 + 2) or 5 (3 + 2)
```

Why does this exist? C was created in the 1970s a time when computer architecture varied greatly. Fewer specifications on a language compiler meant more flexibility of target hardware. This allowed more optimized compilers, simpler language, different CPUs might perform certain operation order more efficiently. Have an unspecified behavior requires the code writer to be more careful and specific.

Q: _What is the heap data structure?_

A: A heap is a tree structure with one parent node called a root. In a max heap all child nodes are less than their parents. So nodes get smaller as the tree grows larger. In a min heap the parent root node is smaller and all the child nodes grow larger as the tree does.

Q: _How does data get stored on the heap?_

A: By using memory allocation functions like calloc, realloc and alloc.

Q: _What about using pointers? Does this store the data on the heap?_

```c
uint8_t* ip;
```

A: Memory is not allocated until the time of assignment so the above statement does not effect memory at all, it's simply a pointer to nothing. During assignment is when the pointer matters. To allocate to the heap the ip would need to use malloc.

_pointers must be initilized to the size of their type:_

```c
uint8_t* p = malloc(sizeof(uint8_t));
```

Q: _what's the difference between these two lines? Functionally and practically:_

```
int* valA = malloc(sizeof(int));
int *valB = malloc(sizeof(int));
```

A: Functionally there are no differences. The compiler treats them the same. But the first line emphasizes that the pointer points to a type of int, while the latter emphasizes that the pointer points to a variable. But both point to the same thing. The important thing to remember is dereferencing only has one syntax: `*valA = 10;` `*valB = 11;`

Q: _What does the macro expand to in the below context? And how does it execute?_

Macro:

```c
#define WAKE_UP() makeCoffee(); drinkCoffee(); // calling WAKE_UP() runs the two functions
```

Context:

```c
if(morning) WAKE_UP;
```

A:

```c
if (morning) makeCoffee(); drinkCoffee(); // makeCoffee executes as part of the if statement. drinkCoffee always executes.
```

Q: _In C, Is it appropriate to call a struct an object? Why or why not?_

It is not appropriate becuase c is not an object oriented languaged and structs lack the fundemental capabilities of OOP Languages: encapsulation (structs are public), no methods on structs, and no inheritance. 


## Common Lang Features:

### pointers

The asterisks can be quite ellusive to newcomers. At a glance it does different things depending on where it is in the syntax:

_pointer declaration_

```c
int *value; // indicates that the pointer is pointing at an integer
value = malloc(sizeof(int)); // creates a pointer to a block of memory
```

_pointer dereferencing_

continuing from the declaration example above

- `*value` returns the int stored at the address held by value.
- `&value` is of type int\*\*. It's the address of the pointer variable.

```c
*value = 10; // stores the value at the location the pointer points to
int *ptrVal = &value; // gets the location of the pointer
int valueAtPtr = *ptrVal; // valueAtPtr is now 10, which is the value of value
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
