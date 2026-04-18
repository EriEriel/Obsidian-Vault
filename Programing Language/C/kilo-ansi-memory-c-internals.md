---
id: kilo-ansi-memory-c-internals
aliases: []
tags: []
---

# Kilo Editor — ANSI Sequences, Memory & C Internals
2026-03-13  #learn #c #systems #memory #terminal #kilo

## what is it
Reference notes from building the kilo text editor — covers ANSI escape sequences, POSIX streams, termios raw mode, Linux memory layout, and the C standard library functions used throughout the project.

## how it works

### ANSI escape sequences
Special byte sequences sent to the terminal to control cursor, screen, and text formatting.

```
\x1b[   →  ESC + CSI (Control Sequence Introducer)
```

| Sequence  | Meaning |
|---|---|
| `ESC[K`   | Clear from cursor to end of line |
| `ESC[2J`  | Clear entire screen |
| `ESC[nC`  | Move cursor right by `n` columns |
| `ESC[nD`  | Move cursor left by `n` columns |
| `ESC[H`   | Move cursor to top-left (row 1, col 1) |
| `ESC[m`   | Set text attributes |

#### text attributes (`ESC[m`)
Combine multiple with semicolons: `ESC[1;4;7m`

| Code | Effect |
|---|---|
| `0` | Reset all |
| `1` | Bold |
| `4` | Underline |
| `5` | Blink |
| `7` | Inverted colors |

#### text colors
| Code | Color |
|---|---|
| `30–37` | Black, Red, Green, Yellow, Blue, Magenta, Cyan, White |
| `39` | Reset foreground (default) |

> Always reset with `ESC[39m` to avoid color bleeding into other output.

---

### POSIX streams & file descriptors
On Unix, everything is a file — including keyboard input and terminal output.

| Name | FD | Description |
|---|---|---|
| `STDIN_FILENO` | `0` | Standard input (keyboard) |
| `STDOUT_FILENO` | `1` | Standard output (terminal) |
| `STDERR_FILENO` | `2` | Standard error |

Kilo reads raw input from `STDIN` and writes directly to `STDOUT`.

---

### termios & raw mode
`termios` controls how the terminal processes input. These `c_iflag` flags are disabled to enter raw mode:

| Flag | Purpose |
|---|---|
| `BRKINT` | Prevents BREAK from sending `SIGINT` |
| `ICRNL` | Stops translating `\r` → `\n` |
| `INPCK` | Disables input parity checking |
| `ISTRIP` | Prevents stripping the 8th bit |
| `IXON` | Disables Ctrl-S / Ctrl-Q flow control |

Disabling these gives the program full control over raw key input.

---

### Linux memory layout (x86_64)
Each process runs in its own virtual address space:

```
High addresses  0x7fff...
┌─────────────────────────────┐
│            Stack            │  grows ↓  (local vars, call frames)
├─────────────────────────────┤
│      Memory-mapped region   │  (shared libs, mmap)
├─────────────────────────────┤
│            Heap             │  grows ↑  (malloc/realloc)
├─────────────────────────────┤
│   BSS / Data (globals)      │
├─────────────────────────────┤
│            Code             │
└─────────────────────────────┘
Low addresses   0x4011...
```

Example addresses:
```
Stack: 0x7ffeefbff5ac
Heap:  0x55555556a2a0
Code:  0x401146
```

Stack and heap grow toward each other — if they collide → out of memory.

---

### stack vs heap

| | Stack | Heap |
|---|---|---|
| Lifetime | Short (function scope) | Long (manual) |
| Size | Fixed | Dynamic |
| Management | Automatic | `malloc` / `free` |
| Speed | Very fast (LIFO) | Slower |
| Use | Local vars, call frames | Large structs, buffers |

---

### C functions used in kilo

#### `memset()` — fill memory with a byte value
```c
void *memset(void *ptr, int value, size_t num);
```
- Writes `num` bytes of `(unsigned char)value` starting at `ptr`
- Returns original `ptr`
- Applied **byte-by-byte** — not as an int

```c
// Kilo usage — initialize syntax highlight array
memset(row->hl, HL_NORMAL, row->rsize);
```

Safe values: `0`, `-1`, small enums. Do NOT use to initialize non-zero ints or floats.

---

#### `memcpy()` — copy memory between locations
```c
void *memcpy(void *dest, const void *src, size_t num);
```
- Exact byte-for-byte copy
- Does NOT handle overlapping memory → use `memmove()` if overlap possible

---

#### `memset()` vs `memcpy()`

| | `memset()` | `memcpy()` |
|---|---|---|
| Purpose | Initialize / reset | Copy |
| Writes | Repeated single byte | Exact bytes from src |
| Use when | Zeroing buffers, setting enums | Copying structs, buffers |

---

#### `memset(&struct, 0)` — when it's safe
```c
memset(&my_struct, 0, sizeof(my_struct));
```
Works because zero bits represent `0` for ints, `NULL` for pointers, `false` for bools on modern systems.

✅ Safe when struct contains: integers, pointers, plain arrays, enums starting at `0`

❌ Unsafe when struct contains:
- `float` / `double` (all-bits-zero ≠ `0.0` guaranteed)
- Enums where `0` is invalid
- Bitfields with non-zero defaults

---

#### `snprintf()` — safe formatted string write
```c
int snprintf(char *buf, size_t size, const char *fmt, ...);
```
- Prevents buffer overflow — truncates at `size`
- If return value ≥ `size`, output was truncated

---

#### `malloc()` / `realloc()`
```c
void *malloc(size_t size);           // allocate on heap
void *realloc(void *ptr, size_t n);  // resize existing allocation
```
- `realloc` may move memory to a new address — always use the returned pointer
- Returns `NULL` on failure — old block still valid if `realloc` fails

---

#### variadic functions
```c
va_list  ap;
va_start(ap, last_fixed_arg);   // initialize
vsnprintf(buf, size, fmt, ap);  // format using va_list
va_end(ap);                     // clean up
```

---

#### `time()`
```c
time_t time(time_t *t);  // returns current time as epoch seconds
```

## example
```c
// Typical kilo pattern — zero-init editor state
struct editorConfig E;
memset(&E, 0, sizeof(E));

// Write escape sequence to clear screen
write(STDOUT_FILENO, "\x1b[2J", 4);
write(STDOUT_FILENO, "\x1b[H", 3);
```

## gotchas
- `memset` is byte-by-byte — never use it to set a multi-byte int to a non-zero value
- `memcpy` does not handle overlapping regions — use `memmove` when source and dest may overlap
- Always reset ANSI color with `ESC[39m` — forgetting causes color to bleed into the shell after exit
- Raw mode must be restored on exit — kilo registers an `atexit()` handler for this

## links
[[c-programming-fundamentals]]
