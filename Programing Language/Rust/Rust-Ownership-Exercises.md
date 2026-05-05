---
id: Rust-Sandboxed
aliases: []
tags:
  - #Rust
  - #Ownership
---

# Rust Ownership exercises

## level 1 - Basic Ownership & Move

### Item 1 - Predict the error

```rust
fn main() {
    let s = String::from("hello");
    let t = s;

    println!("{}", s);
}
```

Task: 
  - Explain why it **fails**
  - Fix it in 2 different ways

Answer:
  It fail because `s` Ownership is already move to `t` so `s` so no longer valid.

  To fix this issue in 2 different ways:

1. Shadowing
```rust
fn main() {
    let s = String::from("hello");
    let t = s;

    println!("{}", t); // now this is valid, but s is still not valid
}
```

2. Using references
```rust
fn main() {
    let s = String::from("hello");
    let t = &s; // assign the reference of s to t

    println!("{}", t);
    println!("{}", s); // s is still valid because we only borrowed it
}
```

### Item 2 - Copy vs Move

```rust
fn main() {
    let x = 10;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
```

Task: 
  - Will this code compile? Why or why not?
  - Explain what changed conceptually.

Answer:
  - This code will compile because `x` is `i32` and being store on stack so it has Copy trait which mean `y` is not reference to `x`, but instead copy the value of 10

## Level 2 - Ownership in Functions

### Item 3 Who own what? 

```rust
fn main() {
    let s = String::from("hello");

    print_string(s);

    println!("{}", s);
}

fn print_string(s: String) {
    println!("{}", s);
}
```

Task:
  - Why does this fail?
  - Fix it without cloning
  - Fix it with cloning

Answer:
  `print_string(s);` is not bind to any variable so and `print_string` function does not return any value so when `s` go in to function the scope is end within function

Fix without cloning:

```rust
fn main() {
    let s = String::from("hello");
    let ptr_s = &s;

    print_string(ptr_s);

    println!("{}", s);
}

fn print_string(s: &str) {
    println!("{}", s);
}

```

Fix with cloning:

```rust
fn main() {
    let s = String::from("hello");

    print_string(s.clone());

    println!("{}", s);
}

fn print_string(s: String) {
    println!("{}", s);
}
```

### Item 4 Return Ownership

```rust
fn main() {
    let s = String::from("hello");

    let s2 = process(s);

    println!("{}", s2);
}

fn process(input: String) -> String {
    // modify and return it
}
```

Task: Append `"world"` inside the function

Answer:

```rust
fn main() {
    let s = String::from("hello");

    let s2 = process(s);

    println!("{}", s2);
}

fn process(mut input: String) -> String {
    input.push_str(", world!");
    input
}
```

## Level 3 - Borrowing

### Item 5 - Borrow instead of move

```rust
fn main() {
    let s = String::from("hello");

    print_length(s);

    println!("{}", s);
}

fn print_length(s: String) {
    println!("{}", s.len());
}
```

Task: Fix without cloning

Answer:

```rust
fn main() {
    let s = String::from("hello");

    print_length(&s);

    println!("{}", s);
}

fn print_length(s: String) {
    println!("{}", s.len());
}
```

### Item 6 - Mutable borrowed

```rust
fn main() {
    let s = String::from("hello");

    add_world(s);

    println!("{}", s);
}

fn add_world(s: &mut String) {
    s.push_str(" world");
}
```

Task: 
  - make this work, only change `main`

Answer:

```rust
fn main() {
    let mut s = String::from("hello");

    add_world(&mut s);

    println!("{}", s);
}

fn add_world(s: &mut String) {
    s.push_str(" world");
}
```

## Level 4 - Borrowing Rules

### Item 7 - Why is this illegal? 

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &mut s;

    println!("{}", r1);
    println!("{}", r2);
}
```

Task:
  - Explain the rule being violated
  - Fix it without removing any `println!`

Answer:
  Rust only allow 1 mutable reference to exist at a time but no limit for immutable reference, but there is both mutable and immutable reference in the same scope so it will not compile

Fix:

1. Remove mutability
```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}", r1);
    println!("{}", r2);
}
```

2. Non-Lexical Lifetimes
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("{}", r1);

    let r2 = &mut s;
    println!("{}", r2);
}
```

### Item 8 - Multiple Mutable references

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

Task:
  - Fix this using scope
  - Fix this without using extra scope

Answer:

1. With scope:
```rust
fn main() {
    let mut s = String::from("hello");

  {
    let r1 = &mut s;

    println!("{}", r1);
  }

    let r2 = &mut s;

    println!("{}", r2);
}
```

2. Without extra scope:
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s.clone();
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

## Level 5 - Slice

### Exercise 9 - First word (index version)

```rust
fn first_word(s: &String) -> usize {
```

Task: 
  - Return the index of the first space

Answer:

```rust
fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  s.len()
}
```

### Exercise 10 - Upgrade to slice

Task: 
  - Same as exercise 9 but using slice instead if index

Answer:

```rust
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}
```

Slight version is better because it tie to original string, Prevent invalid access at compile time also more idiomatic Rust

## Bonus Challenge

### Item 11 - Dangling reference

```rust
fn give_string() -> &String {
    let s = String::from("hello");
    &s
}
```

Task:
  - Fix this properly

Answer:
  Because of `s` is locally scope in `give_string` so when we `&s`, after the function end `s` is already out of scope and become invalid try to access `s` afterward might cause Dangling reference

1. Return the string instead of reference so it can bind to variable
```rust
fn give_string() -> String {
    let s = String::from("hello");
    s
}
```

Another fix might be using lifetime specifier but I don't learning that yet and still it might be another way that I'm not aware of.

### Item 12 - To push even harder, try to answer without compiling

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}", r1);

    let r3 = &mut s;

    println!("{}", r2);
    println!("{}", r3);
}
```

Task:
  - Does this compile under NLL? Why / why not?

Answer:
  - This will not compile because: `r1` and `r2` both are valid reference than we use `r1` it no longer valid, than we try to mutate `s` but `r2` is still valid so `r3` can't mutate it.
