---
id: Rust basic chapter 1-3
aliases: []
tags:
  - #learn
  - #Rust
---

# Rust
2026-03-22  #learn #Rust

## what is it
  The memory safe low-level language that use the concept of borrow checker instead of garbage collector and Raw pointer, This eliminate some class of bug entirely like dangling pointer, stack overflow, out of bound, and memory leak(althou memory leak is still possible but very hard and unlikely)

## how it works
  Introducing to Rust with humble hello world:
```rust
fn main() {
    println!("Hello, world!");
}
```
`fn main` is equivalent to C `int main` it's a main function where program start to run. In Rust there is also `!` for macro like in the `println!` macro function

### cargo the goat package manager of Rust
  Cargo is Rust’s build system and package manager. Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)

Cargo CLI
```bash
cargo --version
cargo new project_name #This has also initialized a new Git repository along with a .gitignore file. 
```

`Cargo.toml` This is a cargo configuration format that also manage dependencies of project, these dependencies package in Rust is also call **crates**

Cargo CLI
```bash
cargo build #Create executable file at target/debug/project_name, for the first time it also create Cargo.lock file that keep track of exact version of dependencies in the project and cargo manage this file itself
cargo run #Compile and produce executable file and execute it at the same command so this is more convenience than cargo build
cargo check #To check if the file compile but does not produce executable and faster than cargo build 
cargo build --release #When project is finish and ready, run with this flag to compile with the optimizations that make code run faster but take more time to compile
```

### Variables and Mutability
Rust variables is immutable by default

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
// This will not compile!!
```

How every the code above will not compile because we try to change `x` value after declaration, to make variables mutable Rust use keyword `mut`

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

`const` keyword use to make variables always immutable and can be declare in any scope include global scope, In rust constant naming convention is use all uppercase with underscore between words, It also always valid for entire program runtime within the scope it declare.

```rust
fn main() {
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
```

**Shadowing** can use to declare same variables name to shadow the previous variable with the same name until it shadowed or scope end, by using shadowing it can perform transformation on the value and it will also be immutable afterward.

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // Output: The value of x in the inner scope is: 12
    }

    println!("The value of x is: {x}"); // Output: The value of x is: 6
}
```

By using `let` it effectively declare the variable again so it can change the type of variables unlike shadowing.

```rust
fn main() {
  let spaces = "   "; // Type: &str
  let spaces = space.len(); // Type: usize
}
```

### Data type
#### Scalar Type
##### Integer
Rust is a statically type language which mean compiler need to know data type at compile time, that has both scalar and compound data type

Table of Integer types in Rust
| Length | Signed | Unsigned |
| --------------- | --------------- | --------------- |
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| Architecture-dependebt| `isize` | `usize` |

Integer literal in Rust
| Number literals | Example |
|--------------- | --------------- |
| Decimal | `98_222` |
| Hex | `0xfff` |
| Octal | `0o77` |
| Binary | `0b1111_0000` |
| Byte(`u8` only) | b`A` |

**Integer Overflow**
  In Rust integer overflow will cause program to `panic!` at runtime in debug mode, but in `--release` Rust will not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a `u8`, the value 256 becomes 0, the value 257 becomes 1, and so on

##### Floating-point
Rust has `f32` and `f64` for the floating-point, with modern age CPUs `f64` has roughly  the same speed as `f32` but has more precision, The default type is `f64`, Floating-point numbers are represented according to the IEEE-754 standard

##### Boolean type
Rust has 2 boolean `true` and `fasle`:

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

##### Character type
Rust’s `char` type is 4 bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emojis; and zero-width spaces are all valid char values in Rust

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

### Compound Type
  Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
#### The Tuple Type
Tuples is a general way to group variety of numbers type together and have a fixed length: Once declared, they cannot grow or shrink in size.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable tup binds to the entire tuple because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

Tuple element can also access directly by using period (.) follow by index of value for access:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

  The tuple without any values has a special name, **unit**. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.

#### The Array Type
  Array in Rust have a fixed length and all element must be the same size and allocated on stack:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5]; // With type annotation and size of 5
    let c = [3; 5]; // Same as let c = [3, 3, 3, 3, 3];
}
```

  To access element in array Rust use index: 

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

  Invalid array access like out of bound access it will cause runtime error and exit the program without execute anything further to ensure the safety

### Function
  Rust use snake case for function name as convention and can be define anywhere as long as it in the same scope with caller:

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

### Statement and Expressions
Rust is an Expressions based-language.
  * Statements are instructions that perform some action and do not return a value.
  * Expressions evaluate to a resultant value.
```rust

fn main() {
    let y = 6; // This is statement and it end with ;
}
```

However This will not compile because `(let y = 6);` is the statement and not return value so can't be bind to x, This will cause compile time error

```rust
fn main() {
    let x = (let y = 6);
}
```

This code will compile because inside { } is an expressions that return value of x than bind it to y

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // This is expressions not the statement so it bind to y
    };

    println!("The value of y is: {y}"); // Output: The value of y is: 4
}
```

**Function with return value**

```rust
fn five() -> i32 { // i32 is annotation for the function return value
    5 // 5 here is already and expressions in Rust so no need to use return, However return can use to return value before the function end
}

fn main() {
    let x = five(); // five() return 5 as expressions so it can bind to x

    println!("The value of x is: {x}");
}
```

Now if there is ; at last line in function the code will not compile because it now is statement not expressions so it not evaluate to any value:

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1; // This is not evaluate to any value so it does not compile
}
```

### Control Flow
Rust has `if` `else if` and `else` for the control flow in Rust, `if` statement need to follow by boolean type

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // if statement can bind to variables because it is an expressions that evaluate to value but all possible return need to be the same type

    println!("The value of number is: {number}");
}
```

**Repetition with Loops**
In Rust there is `loop`, `while` and `for`

The `loop` keyword tells Rust to execute a block of code over and over again either forever or until you explicitly tell it to stop.

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop { // Label outer loop as counting_up with '
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // Inner Loop break here
            }
            if count == 2 {
                break 'counting_up; // Break outer loop with label
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

`while` the condition is true, the loop runs. When the condition ceases to be true, the program calls break, stopping the loop:

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

`for` loop is for when how many time it need to run to complete the task:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

Here is another example of `for` loop with count down, This is inclusive at the start and exclusive at the end so 4 will not show in final output:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

## example
`Cargo.toml` file

```rust
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
```

```rust
// guessing game in Rust
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## links
[Rust 2024 official document chapter 1 - 3](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

