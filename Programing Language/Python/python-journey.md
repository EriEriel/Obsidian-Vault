# Python Learning Journey
2025-10-20  #learn #python #fundamentals

## what is it
Day-by-day log covering Python from scratch. Started 20 October 2025.

## how it works

### day 1 — print, variables, strings
```python
print("Kikirara Vivi")   # strings need quotes
print(6969)              # numbers don't

Item           = "cat"
hololive_member = 6969
```

- Values in `" "` = string
- Use snake_case for variable names: `kikirara_vivi`
- No spaces in names, can't start with a number
- Python is case-sensitive: `a` and `A` are different
- `#` adds a comment

---

### day 2 — operations, reassignment
```python
mana  = 250 + 1          # 251
total = price * amount   # store result in new variable
price = 10               # reassign — last assignment wins
```

- Code runs line by line top to bottom — stops at first bug
- Python is an interpreted language

---

### day 3 — types, booleans, control flow
```python
print(type(3.0))   # <class 'float'>
```

- Division of two integers always returns float
- Type conversion: `int()  str()  float()  bool()`
- `int + str` is invalid: `14 + "km"` fails, `"14" + "km"` works

**Booleans:**
```python
True and True   # True  (anything else is False)
False or False  # False (anything else is True)
```

Comparison always returns Boolean. Boolean values can be stored in variables.

**Loops:**
```python
for i in range(10):
    print("Vivi")      # 10 times — use when count is known

i = 0
while i < 10:
    print("Vivi")
    i = i + 1          # counter — don't forget or it's infinite
```

**If/elif/else:**
```python
if vivi >= 18 and kawaii:
    print("Simp")
elif vivi < 18:
    print("Haiyaaa")
else:
    print("Not Simp")
```

String case methods: `.lower()  .upper()`

---

### day 4 — lists, slicing
```python
number    = [1, 2, 3]
number[0]              # 1
number[0] = 5          # lists are mutable

nums = [1, 2, 3, 4, 5]
nums[0:2]    # [1, 2]  — start inclusive, end exclusive
nums[:2]     # [1, 2]
nums[2:]     # [3, 4, 5]
nums[-1]     # 5  — negative index from end
nums[-3:-1]  # [3, 4]  — order still left to right
```

Strings have indices too but are immutable.

---

### day 5 — functions, infinite loop, string methods
```python
while True:
    ...
    break          # exit
```

```python
.isdigit()      # check if digits
.lower()        # lowercase
.upper()        # uppercase
.capitalize()   # first letter capital
.find("x")      # index of first match, -1 if not found
```

---

### day 6 — list methods, f-strings, def
```python
nums.append(5)        # add to end
nums.insert(3, "x")  # insert at index
nums.pop(0)           # remove at index

f"{value} Fahrenheit"   # f-string

def rect(d1, d2):
    area      = d1 * d2
    perimeter = 2*d1 + 2*d2
    return area, perimeter     # return multiple values

x, y = rect(5, 6)

def nums(a=5):      # default parameter — argument is optional
    return a + 10
```

---

### day 7 — split(), HTML basics
```python
"1 2 3".split()   # ['1', '2', '3']
```

HTML: structure. CSS: style. JavaScript: interactivity.

---

### day 9 — sets
```python
{1, 2, 3}   # set — unordered, no duplicates, uses .add()
[1, 2, 3]   # list — ordered, allows duplicates

{ x for x in N if N.count(x) > 1 }   # find duplicates in list
```

---

### day 10 — advanced slicing, zip, numpy, enumerate
```python
# slice flat list into 3x3 matrix rows
for i in range(3):
    print(A[i*3:(i+1)*3])

# zip pairs items from two lists
total = sum(a * p for a, p in zip(amounts, prices))

# numpy
import numpy as np
total = np.sum(np.array(amounts) * np.array(prices))

# enumerate — index + value in one loop
for i, val in enumerate(my_list):
    print(i, val)
```

Data structure quick reference:
```python
()     # tuple
[]     # list
{}     # dictionary
set()  # set
```

Scientific notation: `1e-9` = 0.000000001

---

### day 11 — abs(), float tolerance
```python
abs(-5)   # 5

# right triangle check with float tolerance
if abs(sides[0]**2 + sides[1]**2 - sides[2]**2) < 1e-9:
    print("right triangle")
```

---

### day 12 — OOP intro
Goal: text-based RPG. Learning OOP first, then file management.
Created student grading program with OOP, pushed to GitHub.

---

### day 13 (14/11/2025) — classes, dict, .items()
```python
class Book:
    def __init__(self, book_id, title, genre, author):
        self.book_id     = book_id
        self.title       = title
        self.genre       = genre
        self.author      = author
        self.is_borrowed = False

class Library:
    def __init__(self):
        self.books_shelf    = {}
        self.borrowed_books = set()

    def add_book(self, book):
        self.books_shelf[book.book_id] = book   # dict[key] = value

    def __str__(self):
        result = "=== Library Books ===\n"
        for book_id, book_obj in self.books_shelf.items():  # .items() = key + value
            result += (
                f"ID: {book_id}\n"
                f"  Title : {book_obj.title}\n"
                f"  Genre : {book_obj.genre}\n"
                f"  Author: {book_obj.author}\n"
                "-----------------------------\n"
            )
        return result
```

## gotchas
- Don't forget `:` after `if`, `elif`, `else`, `for`, `while`, `def`
- Indentation defines blocks — not optional
- `while True` needs a `break` or it runs forever
- Set uses `.add()` not `.append()`
- Division always returns float even between two integers
- Be careful with `or` in if conditions — applies to the whole expression

## links
- [[python-advanced]]
