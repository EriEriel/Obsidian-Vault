---
id: "learn: (9)Rust-ErrorHandling"
aliases: []
tags:
  - learn
  - rust
---

2026-05-09 Init 01:20

## what is it
Rust group error into two categories: recoverable and unrecoverable errors. Recoverable errors are those that can be handled gracefully, such as a file not being found. Unrecoverable errors are those that indicate a bug in the program, such as an out-of-bounds array access. Rust has `Result<T, E>` type for recoverable errors and `panic!` macro for unrecoverable errors.

## how it works
### Unrecoverable Errors with `panic!`
When a Rust program encounters an unrecoverable error, it will call the `panic!` macro, which will print an error message and unwind the stack, effectively terminating the program. This is useful for situations where the program cannot continue to run safely.

#### Unwinding the Stack or Aborting in Response to a Panic
We can choose how to handle a panic by configuring the panic behavior in our Rust program. By default, Rust will unwind the stack, which allows for cleanup of resources. However, we can also choose to abort the program immediately without unwinding, which can be more efficient in certain cases:

```rust
[profile.release]
panic = 'abort'
```

To cause program to we can call `panic!` macro directly or in other cases, such as when we try to access an index that is out of bounds in an array:

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

In order to backtrace the error, we can set the `RUST_BACKTRACE` environment variable to `1`:

```bash
RUST_BACKTRACE=1 cargo run
```

This will output a stack trace that can help us identify where the error occurred in our code so we can start investigating the cause of the panic.

Example of backtrace output:

```
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/std/src/panicking.rs:692:5
   1: core::panicking::panic_fmt
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:273:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:274:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3361:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

At line 6 it points to the line in our code where the panic occurred, which is `v[99]`. This indicates that we attempted to access an index that is out of bounds for the vector `v`, which has a length of 3.

### Recoverable Errors with `Result<T, E>`
Most errors in Rust are recoverable, and the `Result<T, E>` type is used to represent these errors. The `Result` type is an enum that has two variants: `Ok(T)` for successful operations and `Err(E)` for errors. We can use pattern matching to handle the different cases when working with `Result`.

For example, when we try to open a file, we can use `Result` to handle the possibility of an error:

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };
}
```

Here the return type of `File::open` is `Result<File, std::io::Error>`. We use a `match` expression to handle the `Ok` and `Err` cases. If the file is opened successfully, we get a `File` object. If there is an error, we call `panic!` with a message that includes the error details.

#### Matching on Different Errors
The code in the above handles all errors in the same way, but we can also match on different kinds of errors to handle them differently. For example, if we want to create the file if it does not exist, we can do this:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
```

Instead of panicking immediately when we encounter an error, we check the kind of error using `error.kind()`. If the error is `ErrorKind::NotFound`, we attempt to create the file. If the file creation is successful, we get a `File` object. If there is an error during file creation, we panic with a message that includes the error details. For any other kind of error when opening the file, we also panic with a message that includes the error details.

**Alternatives with closures and `unwrap_or_else`** Instead of using nested `match` expressions, we can also use closures and the `unwrap_or_else` method to handle errors more concisely:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
```

About closures and `unwarp_or_else` method more on chapter 13.

#### Shortcuts for Panic on Error: `unwrap` and `expect`
If results are `Ok`, we can call `unwrap` to get the value inside. If the result is an `Err`, the program will panic and print a message that includes the error details. For example:

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

```
thread 'main' panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Similarly, we can use `expect` to provide a custom error message when the program panics:

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

In the same way as `unwrap`, if the file cannot be opened, the program will panic and print the custom error message along with the error details:

```
thread 'main' panicked at src/main.rs:5:10:
hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

`expect` is generally preferred over `unwrap` because it allows us to provide more context about the error, which can be helpful for debugging.

### Propagating Errors with the `?` Operator
Instead of handling errors in the function where they occur, we can also propagate errors to the calling code using the `?` operator. This operator can be used in functions that return a `Result` type. When we use the `?` operator on a `Result`, if the result is `Ok`, it will return the value inside. If the result is an `Err`, it will return the error from the function immediately.

Take a look at the example without `?` but use `match` instead:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

This function attempts to read a username from a file. It first tries to open the file, and if it fails, it returns the error. If it succeeds, it then tries to read the contents of the file into a string. If that operation fails, it also returns the error. If both operations succeed, it returns the username.

Now with using `?` operator:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

The `?` place after returning `Result` type will automatically return the error if the operation fails, which makes the code much cleaner and easier to read. If the file cannot be opened or read, the function will return the error immediately without needing to write explicit `match` statements for each operation.

It function can be even shorter if we use method chaining:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

By using `read_to_string` directly on the result of `File::open`, we can further reduce the amount of code needed to read the username from the file. The `?` operator will still handle any errors that occur during either operation, making the code concise and easy to understand.

But the most convenient way to read a file to a string is to use the `fs::read_to_string` function, which combines the steps of opening the file and reading its contents into a single function call:

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

However this method is less flexible than the previous ones because it does not allow us to handle errors in a more specific way or to perform additional operations on the file before reading its contents. It is a good choice when we just want to read the contents of a file into a string and do not need to do any additional processing.

#### Where to use `?` operator
`?` operator can only be used in functions that return a `Result` type. If we try to use it in a function that does not return a `Result`, we will get a compilation error. For example:

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
```

The main function does not return a `Result` type but return `()` type instead, so using the `?` operator here will result in a compilation error:

```
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:4:48
  |
3 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
4 |     let greeting_file = File::open("hello.txt")?;
  |                                                ^ cannot use the `?` operator in a function that returns `()`
  |
help: consider adding return type
  |
3 ~ fn main() -> Result<(), Box<dyn std::error::Error>> {
4 |     let greeting_file = File::open("hello.txt")?;
5 +     Ok(())
  |

For more information about this error, try `rustc --explain E0277`.
error: could not compile `error-handling` (bin "error-handling") due to 1 previous error
```

`?` operator can also be used in functions that return `Option` type, where it will return `None` if the operation fails instead of returning an error. For example:

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

`line()` method returns an iterator over the lines of the string, and `next()` returns the first line as an `Option<&str>`. If there are no lines, it will return `None`, and the `?` operator will cause the function to return `None` immediately. If there is a line, it will then call `chars().last()` to get the last character of that line, which also returns an `Option<char>`. If there are no characters in the line, it will return `None`, and if there is a character, it will return `Some(char)`.

However, `?` will not convert `Result` to `Option` or vice versa. If we try to use `?` to convert a `Result` to an `Option`, we will get a compilation error. In those case to do a conversion, we can use the `ok()` method to convert a `Result` to an `Option`, or the `ok_or()` method to convert an `Option` to a `Result`.

The main function can also be modified to return a `Result` type, which would allow us to use the `?` operator directly in the main function:

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

The `Box<dyn Error>` type is a trait object that can represent any error type that implements the `Error` trait. This allows us to return different types of errors from the main function without needing to specify a concrete error type.

Like in C, the executable will exit with a nonzero exit code if the main function returns an error, which indicates that the program encountered an error during execution. If the main function returns `Ok(())`, the program will exit with a zero exit code, indicating that it completed successfully.

### To `panic!` or not to `panic!`
**Default to `Result`** This gives the caller of the function the choice of how to handle the error, whether to propagate it further up the call stack or to handle it immediately. Calling `panic!` right away would take that choice away from the caller and force them to deal with the panic, which may not be desirable in all cases.

#### When `panic!` (or `unwrap` and `expect`) is appropriate
Examples, prototypes, and tests — .unwrap() is acceptable as a placeholder. In tests especially, we want a panic to mark the test as failed.
When we know more than the compiler — if you can guarantee a Result will be `Ok` through logic the compiler can't see, `.expect("reason")` is fine. The classic example:

```rust
let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
```

The String is obviously a valid IP address, so we can use `expect` to panic if it is not. This is a case where we know more than the compiler, and we want to provide a clear message about why the panic occurred if it does happen.

#### When to `panic!` in Real Code
`panic!` when code git **bad state** — a broken assumption or violated contract — specifically when:
* The failure is unexpected, not a routine possibility (like bad user input)
* Code cannot safely continue from that point
* There's no sensible type to encode the constraint

Also `panic!` when security is at risk — if continuing execution could lead to a security vulnerability, it's better to panic than to allow the program to continue in an unsafe state like out of bounds access or integer overflow.

#### When to return `Result` in Real Code
When failure is **expected and recoverable** — a parser getting malformed data, an HTTP rate limit response. These are foreseeable conditions the caller should decide how to handle.

#### Custom type for validation
Back to guessing game in chapter 2:

```rust
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
        // --snip--
}
```

It would be tedious to write this kind of validation logic every time we want to get a number from user input. Instead, we can create a custom type that encapsulates the validation logic and provides a clean interface for working with valid numbers:

```rust
pub struct Guess { value: i32 }

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 { self.value }
}
```

This way we provide a clear contract for what constitutes a valid guess, and we can use the `Guess` type throughout our code to ensure that we are always working with valid numbers. If someone tries to create a `Guess` with an invalid value, the program will panic with a clear message about why the panic occurred.

We also implement `value()` method to allow access to the inner value of the `Guess` struct, This is call *gatter* method, which is a common pattern in Rust for providing controlled access to the private fields of a struct. By using a getter method, we can ensure that the value of the `Guess` struct is always valid and that any code that uses it can rely on that validity and can't access the inner value directly without going through the validation logic.

## gotchas

## links
[Rust doc Error-handling](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)
[[(1-3)Rust-basic]]
