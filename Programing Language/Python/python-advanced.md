# Python Advanced Topics
2025-11-01  #learn #python #advanced #functional #oop

## what is it
Deeper Python — lazy vs eager evaluation, generators, lambdas, decorators, scope, namespaces, string operations, walrus operator, async, and exception handling.

## how it works

### advanced while loop
```python
while condition:
    break      # terminate loop entirely
    continue   # skip current iteration
else:
    # runs only when loop ends naturally, NOT via break
    ...
```

---

### lazy vs eager

**Eager** — compute everything immediately:
```python
nums = [x * 2 for x in range(5)]   # [0, 2, 4, 6, 8] stored in memory now
```

**Lazy** — compute on demand:
```python
nums = (x * 2 for x in range(5))   # <generator object> — just a recipe, nothing computed yet
```

Use lazy when: data is large, you don't need everything, pipelines (`map -> filter -> zip`), reading files/APIs.
Use eager when: data is small, need random access, need to reuse values.

---

### generators
```python
gen = (i for i in range(3))
print(*gen)    # 0 1 2  — unpack with *

next(gen)      # compute one value at a time
for x in gen:  # or consume with loop
    print(x)
```

Unpack rules: `*iterable` → tuple, `**mapping` → dictionary.

---

### lambda, map, filter
```python
# lambda
lambda x, y: x * y
print((lambda x, y: x * y)(3, 2))   # 6

# map — apply function to each element, returns lazy iterator
list(map(lambda x: x * 2, [1, 2, 3]))   # [2, 4, 6]

# filter — keep elements that pass the test
list(filter(lambda x: x > 5, [3, 4, 5, 6, 7]))   # [6, 7]
```

---

### class decorators

**@classmethod** — takes `cls` as first arg, can access/modify class-level data:
```python
@classmethod
def method(cls): ...
```

**@staticmethod** — regular function grouped inside class, no `self` or `cls`:
```python
@staticmethod
def method(): ...
```

**@property — getter/setter/deleter:**
```python
class User:
    def __init__(self, email):
        self._email = email       # _name = internal use convention

    @property
    def email(self):              # getter
        return self._email.lower()

    @email.setter
    def email(self, value):       # setter — validate before storing
        if "@" not in value:
            raise ValueError("Invalid email")
        self._email = value

    @email.deleter
    def email(self):              # deleter
        del self._email
```

`_name` (single leading underscore) = internal use only by convention, not enforced.

---

### type hints
```python
language:    str   = "Python"
number:      int   = 42
coefficient: float = 2.87

colors: dict[str, str] = {"red": "#FF0000"}
colors: dict[str, tuple[int, int, int]] = {"Red": (255, 0, 0)}
```

---

### parallel assignment and unpacking
```python
is_authenticated = is_active = is_admin = False   # parallel assignment

numbers = (1, 2, 3)
a, b, c = numbers          # tuple unpacking

numbers = (1, 2, 3, 4, 5)
a, *b, c = numbers         # a=1, b=[2,3,4], c=5

a, b = b, a                # swap
```

---

### walrus operator `:=`
Statement = performs action, does NOT return a value.
Expression = evaluates to a value.

`:=` assigns AND returns the value — usable in conditions:
```python
if (data := input()) != None:
    print(data)

while (line := input()) != "quit":
    print(line)

if (n := len(data)) > 10:
    print(f"Too long: {n}")
```

---

### scope — LEGB rule
Python searches: **L**ocal -> **E**nclosing -> **G**lobal -> **B**uilt-in

```python
# nonlocal — access enclosing function's variable
def outer():
    count = 10
    def inner():
        nonlocal count
        count += 1
    inner()

# global — modify file-level variable from inside a function
def set_timeout(seconds):
    global _timeout
    _timeout = seconds
```

Python looks for **names** in namespaces, not variables:
```python
Namespace  =  dict of name -> object
Scope      =  visibility rules
LEGB       =  search order
```

---

### string operations
```python
a = "Hello World"
a[:2]            # "He"

for i in "Python":
    print(i)

if "H" in "helloworld": ...
if "H" not in "helloworld": ...
```

**Interpolation:**
```python
f"My name is {name}"                       # f-string (preferred)
"My name is {name}".format(name="Malenia") # .format() for dynamic templates
```

**Formatting mini language:**
| Type | Description | Example | Output |
|---|---|---|---|
| `s` | String | `f"{'test':<10}"` | `'test      '` |
| `d` | Decimal int | `f"{123:05d}"` | `'00123'` |
| `f` | Float | `f"{123.456:.2f}"` | `'123.46'` |
| `%` | Percentage | `f"{0.45:%}"` | `'45.00000%'` |
| `<` | Left align | `f"{'left':<10}"` | `'left      '` |
| `>` | Right align | `f"{'right':>10}"` | `'     right'` |
| `^` | Center | `f"{'center':^10}"` | `'  center  '` |
| `,` | Thousands | `f"{1000000:,}"` | `'1,000,000'` |

**Encoding:**
- UTF-8 — standard encoding for Python and the web. Supports all languages including Thai and Japanese.
- ASCII — 128 characters only: English, numbers, symbols. No Thai, no Japanese.

---

### async
```python
import asyncio

async def fetch():
    await some_io_operation()
# allows two processes to run in parallel instead of sequentially
```

---

### exception handling
```python
try:
    risky_operation()
except ValueError as e:
    print(e)
except (TypeError, KeyError):
    ...
else:
    # runs if NO exception was raised
    ...
finally:
    # always runs — use for cleanup
    ...

assert condition, "message"   # raises AssertionError if condition is False
```

## example
```python
# lazy pipeline — double all numbers then keep those > 5
result = list(
    filter(lambda x: x > 5,
    map(lambda x: x * 2, range(10)))
)
# [6, 8, 10, 12, 14, 16, 18]
```

## gotchas
- Variables are local by default — use `global` or `nonlocal` to modify outer scope
- `nonlocal` goes up one level only — to the enclosing function, not global
- `@property` makes a method look like an attribute — call without `()` from outside
- Generators are consumed once — iterating again yields nothing
- Walrus `:=` is only useful when you need the value immediately after assigning

## links
- [[python-journey]]
- [[javascript]]
