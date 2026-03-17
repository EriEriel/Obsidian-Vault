# C Programming Fundamentals
2025-10-01  #learn #c #systems #lowlevel

## what is it
Core C language reference — compilation, data types, operators, control flow, arrays, strings, pointers, memory, functions, structs, and bitwise operations.

## how it works

### compilation
```bash
gcc --version                    # check GCC version
clang --version                  # check Clang version

clang -std=c17 main.c            # compile with Clang (C17), outputs a.out
gcc   -std=c17 main.c            # compile with GCC
clang -std=c17 main.c -o potato  # compile to named executable

./a.out                          # run the output
```

GCC vs Clang: GCC offers maximum optimization and broader language support but slower compile and cryptic errors. Clang offers faster compile, superior error messages, and modular design.

**ELF (Executable and Linkable Format)** — binary format on Linux/Unix. Stores machine code, CPU target metadata, section layout, relocation info, dynamic linking info, and string resources.

---

### hello world breakdown
```c
#include <stdio.h>       // STanDard Input/Output — copy-paste header before compiling

int main(void) {         // entry point; int = returns integer to OS
                         // void = takes no parameters (different from main())
  printf("Hello World!");
  return 0;              // 0 = success, returned to OS
}                        // every statement ends with ;
```

---

### escape sequences
```
\n   newline
\t   horizontal tab
\\   backslash
\"   double quote
```

---

### data types
```c
int     // whole numbers: 123, -123          (2-4 bytes)
float   // decimal, ~6-7 digit precision     (4 bytes)
double  // decimal, ~15 digit precision      (8 bytes)
char    // single character: 'a', 'b'        (1 byte)
```

Extended integer types:
| Type | Size | Format specifier |
|---|---|---|
| `short int` | 2 bytes | `%hd` |
| `unsigned int` | 2 or 4 bytes | `%u` |
| `long int` | 4-8 bytes | `%ld` |
| `long long int` | 8 bytes | `%lld` |
| `unsigned long int` | 4-8 bytes | `%lu` |
| `unsigned long long int` | 8 bytes | `%llu` |
| `long double` | 8/12/16 bytes | `%Lf` |

> Sizes vary by system (32-bit vs 64-bit) and compiler.

---

### format specifiers
```
%d    whole number (int)
%f    floating point
%c    character
%s    string
%p    pointer address
%zu   size_t (sizeof() return type)
```

Decimal precision: `printf("%.5f", num)` — 5 decimal places.
Scientific notation: `float num = 35e5` gives `3500000`.

---

### variables
```c
int num = 10;             // declare + assign
int x;                    // declare only, assign later
x = 5;
int a = 5, b = 6, c = 7; // multiple same-type on one line

// assign from another variable
int other = num;

// constant — read-only, uppercase by convention
const int BIRTHYEAR = 2000;
```

---

### operators

**Arithmetic:** `+  -  *  /  %  ++  --`

Pre-increment (`++x`) adds 1 first then uses value. Post-increment (`x++`) uses value then adds 1. Same for decrement.

**Assignment:** `+=  -=  *=  /=  %=`

**Bitwise assignment:** `&=  |=  ^=  >>=  <<=`

**Comparison:** `==  !=  >  <  >=  <=`

**Logical:** `&&  ||  !`

**Order of operations (high to low):**
```
()            parentheses
*, /, %       multiply, divide, modulus
+, -          add, subtract
>, <, >=, <=  comparison
==, !=        equality
&&            logical AND
||            logical OR
=             assignment
```

---

### booleans
```c
#include <stdbool.h>

bool result = true;
bool other  = false;
// prints as 1 (true) or 0 (false) with %d
```

---

### control flow
```c
// if / else if / else
if (x > 10) { ... }
else if (x == 5) { ... }
else { ... }

// switch — needs break to prevent fallthrough
switch (expression) {
  case 1: ...; break;
  case 2: ...; break;
  default: ...;
}

// ternary
variable = (condition) ? expressionTrue : expressionFalse;
```

---

### loops
```c
while (condition) { }

do { ... } while (condition);   // executes at least once

for (int i = 0; i < 5; i++) {
  printf("%d\n", i);
}

// break  = stop loop completely
// continue = skip this iteration, keep looping
```

---

### arrays
```c
int myArray[] = {1, 2, 3, 4, 5};   // index starts at 0

// size
int len = sizeof(myArray) / sizeof(myArray[0]);

// loop
for (int i = 0; i < len; i++) {
  printf("%d\n", myArray[i]);
}

// 2D array
int matrix[2][3] = { {1, 2, 3}, {4, 5, 6} };
// use nested loops to iterate
```

---

### strings
```c
// no string type in C — use char array
char hello[] = "hello world";

// equivalent explicit form — note the null terminator
char hello[] = {'H', 'e', 'l', 'l', 'o', '\0'};

// \0 tells C where the string ends; format specifier: %s
```

`<string.h>` functions:
```c
strlen(str)          // length
strcat(s1, s2)       // append s2 onto s1
strcpy(s1, s2)       // copy s2 into s1
strcmp(s1, s2)       // compare; returns 0 if equal
```

User input:
```c
scanf("%s", name);                    // single word
fgets(name, sizeof(name), stdin);     // multiple words (safer)
```

---

### memory and pointers

**Memory address:**
```c
int x = 10;
printf("%p", &x);   // prints 0x7ffda0aadd94 — & is the reference operator
```

**Pointer:**
```c
int x = 10;
int *ptr = &x;         // ptr stores address of x
printf("%d", *ptr);    // dereference: get the value at that address
```

**Pointers and arrays:**
```c
// array name IS a pointer to its first element
int myArray[5] = {1, 2, 3, 4, 5};
int *ptr = myArray;

printf("%d", *ptr);        // 1
printf("%d", *(ptr + 1));  // 2 — jumps by sizeof(int) = 4 bytes

*myArray = 13;             // modify via pointer
```

**Pointer arithmetic:**
Pointers jump by the size of their type — 4 bytes for `int`, 1 byte for `char`.
```c
int *start = &myNumbers[1];  // points to element 20
int *end   = &myNumbers[4];  // points to element 50
printf("%d", end - start);   // 3 — elements between them
```

**Double pointer:**
```c
int myNum  = 10;
int *ptr   = &myNum;
int **pptr = &ptr;
printf("%d", **pptr);   // 10
```

---

### functions
```c
// good practice: declare above main, define below main
int myFunction(int x, int y);   // declaration

int main(void) {
  int result = myFunction(5, 3);
  printf("Result = %d", result);
  return 0;
}

int myFunction(int x, int y) {  // definition
  return x + y;
}
```

Scope: local (inside function), global (outside all functions). If same name exists at both levels, local wins.

`<math.h>` functions: `sqrt()  ceil()  floor()  pow(x,y)`

**Inline function:**
```c
inline int add(int a, int b) { return a + b; }
// copies body at call site instead of jumping — good for small, frequent functions
```

**Recursion:**
```c
int factorial(int n) {
  if (n > 1) return n * factorial(n - 1);
  else       return 1;
}
// must have a base case to avoid infinite recursion
```

**Function pointer:**
```c
int (*ptr)(int, int) = add;
int result = ptr(5, 3);
```

**Arrow operator (`->`):** Access struct/union members through a pointer.
```c
bob_ptr->age = 30;   // same as (*bob_ptr).age = 30
```

---

### structs
```c
struct Person {
  char name[50];
  int  age;
};

struct Person bob;
struct Person *bob_ptr = &bob;

bob.age      = 30;   // dot operator — direct access
bob_ptr->age = 30;   // arrow operator — via pointer
```

---

### memory layout (x86_64)
```
High 0x7fff...
+-----------------------------+
|            Stack            |  grows down  (local vars, call frames)
+-----------------------------+
|      Memory-mapped region   |  (shared libs, mmap)
+-----------------------------+
|            Heap             |  grows up    (malloc/realloc)
+-----------------------------+
|   BSS / Data (globals)      |
+-----------------------------+
|            Code             |
+-----------------------------+
Low  0x4011...
```

Stack — short-lived, fixed, automatic, fast (LIFO).
Heap — long-lived, dynamic, manual (`malloc`/`free`), flexible.

---

### standard library

**`<stdio.h>`**
```c
int snprintf(char *buf, size_t n, const char *fmt, ...);
// safe formatted write — truncates at n; if return >= n, output was truncated
```

**`<stdlib.h>`**
```c
void *malloc(size_t size);           // allocate heap memory
void *realloc(void *ptr, size_t n);  // resize; may move memory; returns new ptr or NULL
```

**`<string.h>`**
```c
void *memset(void *ptr, int val, size_t n);           // fill n bytes with val (byte-by-byte)
void *memcpy(void *dest, const void *src, size_t n);  // copy n bytes src->dest (no overlap)
```

**Variadic:**
```c
va_list ap;
va_start(ap, last_arg);
vsnprintf(buf, size, fmt, ap);
va_end(ap);
```

```c
time_t time(time_t *t);   // current time as epoch seconds
```

---

### bitwise flags
```c
#define FLAG_A (1 << 0)   // 0001
#define FLAG_B (1 << 1)   // 0010
#define FLAG_C (1 << 2)   // 0100

int flags = FLAG_A | FLAG_C;   // 0101 — multiple ON at once

if (flags & FLAG_B) { }    // check: is FLAG_B on?  — uses & not &&
flags |= FLAG_B;            // turn ON
flags &= ~FLAG_B;           // turn OFF
flags ^= FLAG_B;            // toggle
```

`&` (bitwise AND) checks bit presence. `&&` (logical AND) checks boolean truth. Bitmasks need `&`.
A bitmask is a boolean array packed into bits — `flags & X` means "is X enabled?"

## gotchas
- `main(void)` is not the same as `main()` in C — always use `void` for empty parameter lists
- `memset` is byte-by-byte — never use it to initialize multi-byte ints to non-zero values
- `memcpy` does not handle overlapping memory — use `memmove` when regions may overlap
- `switch` requires `break` — omitting it causes silent fallthrough to the next case
- Array name is already a pointer — `myArray` and `&myArray[0]` are the same address
- `realloc` may move memory — always use the returned pointer, not the original
- Pre vs post increment matters in expressions: `++x` increments before use, `x++` after

## links
- [[kilo-ansi-memory-c-internals]]
- [[c-memory-management]]
