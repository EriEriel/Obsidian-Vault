---
id: Rust-Ownership
aliases: []
tags:
  - rust
  - ownership
  - learn
---

2026-04-18 Init 01:51

## what is it
Rust manage memory with `ownership system` which is a set of rule to check memory at compile time so it not effect performance at runtime at all.

### Stack and heap recap
  Variable that know the size at compile time will be store on stack, Variable with dynamic size need to be store on heap instead. When allocating memory to the heap it will usually return pointer to the address and that pointer will be store on stack because pointer has a fixed size, access memory on heap is generally slower than stack due to it scatter on the heap while on stack memory is ordered.

## how it works
### Ownership Rule
* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

### Scope
  Variable will remain valid until it goes out of scope: 

```rust
fn main() {
  {
     let x = "Hello"; // This is a string literal type, and know size at compile time so it can be store on the stack

    // Do some stuff with s here
  }
}
```

### The string Type
   Beside from string literal rust also have another string type that allocate into heap and size can be change:

```rust
fn main() {
  { 
    let mut s = String::from("hello"); // This implement the request for memory allocation

      s.push_str(", world!"); // push_str() appends a literal to a String

      println!("{s}"); // this will print `hello, world!`
  } // s is already out of scope and no longer valid here
}
```

* The memory must be requested from the memory allocator at runtime.
* It need a way of returning this memory to the allocator when we’re done with our String.

When variable out of scope rust will call `drop` function to free a memory without a need to use GC(garbage collector) or have to manually free it.

### Basic of Rust copy trait and move semantic

Look at the following example:

```rust
fn main() {
{
    let s1 = String::from("hello, world");
    let s2 = s1;

    println!(s1);
  }
}
```

However, this will not compile because `s1` is already move to `s2` and no longer valid, the line `let s2 = s1;` mean `s2` now copy the pointer of `s1` which store the address on heap, length, and capacity without being copy the actual data on heap this is being use with any dynamic size data that can change on runtime like  `String`, `Vector`, and `Box`

The data on stack instead of ownership they have copy trait which mean:

```rust
fn main() {
  let x = 5;
  let y = x;

  println!("x");
  println!("y");
}
```

Both are valid and can be print because the variables on stack have a copy trait so now y is copy value of 5 and independent from x, any copy operation like this on Rust can assume that it inexpensive and don't have impact on performance and Rust will never implicitly make deep copy of any data on stack at all.

To deep copy data on heap Rust can use `.clone()` method:

```rust 
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}
```

**Ownership and Function**
  Take a look at example below:

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
```

  The function ownership is similar to variable ownership, when we pass a variable to a function it will move the ownership of that variable to the function and when the function end the variable will go out of scope and drop, if we want to use that variable after the function call we can return it back from the function or use reference.

```rust
fn main() {
    let s1 = String::from("hello");

    let s2 = takes_and_gives_back(s1);

    println!("{s2}");
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}
```

  Form the above example the ownership of `s1` move to `takes_and_gives_back` function and then return back to `s2` so we can use it after the function call and `s1` is no loner valid.

To return multiple value from a function we can use tuple:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

### Reference and Borrowing
  Instead of return ownership back from a function we can use reference to borrow a value without taking ownership of it:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

  In this example we pass a reference of `s1` to the function `calculate_length` and it can use that reference to calculate the length without taking ownership of `s1` so we can still use `s1` after the function call. Although we can use reference to borrow a value but we can't modify it because it's immutable by default, if we want to modify it we need to use mutable reference:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

  In this example we pass a mutable reference of `s` to the function `change` and it can modify the value of `s` by appending ", world" to it.

  To prevent data race Rust have a rule that we can only have one mutable reference to a value at a time or any number of immutable reference but not both at the same time:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, {r3}");
}
```

  In this example we have two immutable reference `r1` and `r2` to `s` and one mutable reference `r3` to `s` which is not allowed because it can cause data race.

  If we want to mutate a value the previous immutable reference will no longer valid and we can create a mutable reference:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{r1}, {r2}"); // we can use r1 and r2 here and both r1 and r2 are no longer valid after this point

    let r3 = &mut s; // no problem

    println!("{r3}"); // we can use r3 here
}
```

  Rust also don't allow to have multiple mutable reference to the same value at the same time:

```rust
fn main() {
  let mut s = String::from("hello");

  let r1 = &mut s; // no problem
  let r2 = &mut s; // BIG PROBLEM
}
```

  In order to use `r2`, `r1` need to be out of scope first, we can create a new block to make `r1` go out of scope before creating `r2`:

```rust
fn main() {
  let mut s = String::from("hello");

  {
    let r1 = &mut s; // no problem
  } // r1 goes out of scope here, so we can make a new reference with no problems.

  let r2 = &mut s; // no problem
}
```

  To prevent dangling pointer Rust also don't allow to return reference to a value that will go out of scope:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

reference_to_nothing: &String {
    let s = String::from("hello");

    &s
}
```

  In this example the function `dangle` return a reference to a value `s` that will go out of scope when the function end so it's not allowed because it can cause dangling pointer.

### The slice type
  Slice is a reference to a contiguous sequence of elements in a collection, it can be used to reference a part of a string or an array without taking ownership of it:

  Let's take a look at the following example:

```rust
fn main() {
    let s = String::from("hello world");

    first_word(&s); 
    println!("The first word is: {}", first_word(&s));

    first_word_slice(&s);
    println!("The first word is: {}", first_word_slice(&s));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

  `first_word` function return the index of the first space in the string, while `first_word_slice` function return a slice of the string that contains the first word. Both functions take a reference to a string and do not take ownership of it, so we can still use the original string after calling these functions.

  Without slice we need to return the index of the first word and then use that index to get the first word from the original string, but with slice we can return a reference to the first word directly without needing to return the index and then use it to get the first word.

  Slice syntax can also be used with array:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]); // This is Rust marco that will check if the slice is equal to the array [2, 3] and panic if it's not

    println!("slice: {:?}", slice);
}
```

  In this example we create a slice of the array `a` that contains the elements from index 1 to index 2 (not including index 3) and print it out. The output will be `slice: [2, 3]`.

## gotchas

* Non-Lexical Lifetimes (NLL), example:

```rust
fn main() {
 let mut s = String::from("hello");

    let r1 = &s; 
    println!("{}", r1); // r1 is last used here.

    // Because r1 is never used again, the borrow checker 
    // considers it "dropped," even though we are still inside the scope.
    
    let r2 = &mut s; // This is valid! 
    println!("{}", r2);
}
```

This show that the borrow checker is smart enough to know that `r1` is no longer used after the first `println!` and allows us to create a mutable reference `r2` to `s` without any issues.

* The Copy Trait is not automatic ,example:

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

fn main() {
    let p1 = Point { x: 0, y: 0 };
    let p2 = p1; // This MOVES p1 to p2. 
    // println!("{:?}", p1); // ERROR: p1 is no longer valid.
}
```

  Even though `Point` is a simple struct that contains only `i32` fields, it does not implement the `Copy` trait by default. This means that when we assign `p1` to `p2`, it moves the ownership of `p1` to `p2`, and `p1` is no longer valid. If we want to make `Point` implement the `Copy` trait, we can add the `#[derive(Copy, Clone)]` attribute to the struct definition.

## links
[[kilo-editor-ansi-sequences-memory-c-internals]]
[[(1-3)Rust-basic]]
