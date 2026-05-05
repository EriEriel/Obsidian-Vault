---
id: "learn: Rust-Enums"
aliases: []
tags:
  - learn
  - rust
  - enums
---

2026-04-26 Init 16:55

## what is it
While structs are used to create custom data types that group together related data, enums (short for "enumerations") are used to define a type that can be one of several different variants. Each variant can have its own associated data, making enums a powerful tool for modeling complex data structures and handling different cases in a type-safe way.

## how it works
### Defining an enum
Example of defining an struct to represent an IP address:
```rust
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    enum IpAddrKind {
        V4,
        V6,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("
```

Example of defining an enum to represent different types of IP addresses:
```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
```

From the above example, we can see that the `IpAddr` enum has two variants, `V4` and `V6`, each of which can hold a `String` value representing the IP address. This allows us to create instances of `IpAddr` that can represent either an IPv4 or an IPv6 address without needing to define a separate struct for each type.

There is another advantage of using enums: Each variant of an enum can have different types and amounts of associated data. This means that we can use enums to represent a wide variety of data structures and cases without needing to create multiple structs or use complex data types.

Example of defining an enum to represent different types of messages:
```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
```

Another advantage of using enums over structs because of with structs, we can't easily define a function that can operate on multiple variants of a struct without using traits or other complex features. With enums, we can define methods directly on the enum type that can operate on all variants, making it easier to write code that can handle different cases in a type-safe way.

As structs enum can also have methods defined on them. This allows us to define behavior that is specific to the enum type and can operate on all variants of the enum.

Example of defining methods on an enum:
```rust
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
        println!("Message: {:?}", self);
        }
    }

    fn main() {
        let m = Message::Write(String::from("hello"));
        m.call();
    }
```

### The option Enum
In many languages, like JavaScript, C++, Java, etc., there is a concept of "null" or "undefined" to represent the absence of a value. However, in Rust, there is no such concept. Instead, Rust provides the `Option` enum to represent the possibility of a value being present or absent. It use to prevent **The billion-dollar mistake** in which the code trying to access a value that is null or undefined, which can lead to runtime errors and crashes.

The Definition of the `Option` enum is as follows:
```rust
    enum Option<T> {
        None,
        Some(T),
    }
```

- `None`: Represents that there is no value. It is similar to null, but it is not a "trap" — you are explicitly choosing to handle the case where data is missing.

- `Some(T)`: Represents that there is a value, and that value is stored inside. T is a generic placeholder, meaning it can hold any type (an integer, a string, a struct, etc.).

Because both `Option<T>` and `T` are different types, Rust's type system forces us to handle the case where there might not be a value, which helps prevent bugs and makes our code more robust.

Example of using the `Option` enum:
```rust
  enum Option<T> {
      None,
      Some(T),
  }

  fn main() {
      let some_number = Some(5);
      let some_string = Some("a string");

      let absent_number: Option<i32> = None;

      let x: i8 = 5;
      let y: Option<i8> = Some(5);

      let sum = x + y;
  }
```

The code above will not compile because we cannot add an `i8` and an `Option<i8>`. We need to handle the case where `y` might be `None` before we can perform the addition. This forces us to write code that is more robust and less prone to runtime errors.

`match` is a Rust powerful control flow operator that allows us to compare a value against a series of patterns and execute code based on which pattern matches. It is often used in conjunction with enums to handle different cases in a type-safe way.

Example of `match` with coin sorting:
```rust
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
```

To run multiple line of `match` code, we can use curly braces to create a block of code for each arm of the `match` expression. This allows us to execute multiple statements for each pattern.

Example of `match` with multiple lines of code:
```rust
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
```

### Pattern that bind to values
Match arms can also bind to the values that match the pattern. This allows us to use the values in the code that runs for that arm of the `match` expression.

Example of pattern that bind to values:
```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

If we use `value_in_cents(Coin::Quarter(UsState::Alaska))` as argument, `UsState::Alaska` will be bound to the variable `state` in the `match` arm for `Coin::Quarter`, allowing us to use it in the code that runs for that arm.

### `Option<T>` match pattern
When we use `match` with the `Option<T>` enum, we can handle both the `Some` and `None` cases explicitly. This allows us to write code that is more robust and less prone to runtime errors.

Example of `Option<T>` match pattern:
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let intSix = plus_one(Some(x));

    println!("six: {:?}", six); // Output: six: Some(6)
    println!("none: {:?}", none); // Output: none: None
}
```

Since `plus_one` takes an `Option<i32>` as an argument, we need to handle both the `Some` and `None` cases in the `match` expression. If we pass `Some(5)` to `plus_one`, it will return `Some(6)`. If we pass `None`, it will return `None`. This forces us to write code that is more robust and less prone to runtime errors, as we are explicitly handling the case where there might not be a value.

**`match` is Exhaustive** match in Rust must be exhaustive, meaning that all possible cases must be handled. If we forget to handle a case, the compiler will give us an error. This is a powerful feature that helps prevent bugs and ensures that our code is robust and impossible for *billion-dollar mistake*.

Example of non-exhaustive match:
```rust
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            // Missing arm for Coin::Quarter
        }
```

Example of `option<T>`:
```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
```

Both of code example above will not compile because the `match` expression is not exhaustive. We need to handle all possible cases for the `Coin` enum which is `Quarter` and the `Option<i32>` enum which is `None` to make the code compile successfully. Compiler even know what match are missing and give us a hint on how to fix it.

### Catch all pattern with `_` placeholder
Because match expressions must be exhaustive, we can use the `_` placeholder to catch all values and not bind to that values. This allows us to write code that is more concise and easier to read when we don't care about certain cases.

Let's say there is a game where you roll a dice and if you roll a 3, you get to add a fancy hat to your character, if you roll a 7, you get to remove the fancy hat, and for any other roll, you have to reroll. We can use a `match` expression with a catch-all pattern to handle this logic.

Example of catch all pattern with `_` placeholder:
```rust
let dice_roll = 9;
  match dice_roll {
      3 => add_fancy_hat(),
      7 => remove_fancy_hat(),
      _ => reroll(),
  }

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

Because `match` is exhaustive, we need to handle all possible values of `dice_roll`. By using the `_` placeholder, we can catch all values that are not 3 or 7 and call the `reroll()` function for those cases. This allows us to write code that is more concise and easier to read when we don't care about certain cases.

Now let's change the game rules a bit if you roll any other value other than 3 or 7, nothing else happens, we can simply use the `_` placeholder and using unit values:
```rust
let dice_roll = 9;
  match dice_roll {
      3 => add_fancy_hat(),
      7 => remove_fancy_hat(),
      _ => (),
  }

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

Now if it matches 3 or 7, it will call the respective functions, but if it matches any other value, it will do nothing because the `_` pattern is matched and the unit value `()` is returned, which represents an empty tuple and does not have any meaningful value.

### Concise Control Flow with if let and let...else
`if let` syntax let's us use less verbose way to handle the values that match one pattern while ignoring the rest

Consider the below example without `if let`:
```rust
let config_max = Some(3u8);
  match config_max {
      Some(max) => println!("The maximum is configured to be {max}"),
      _ => (),
  }
```

Now with `if let` the code behave the save way as `match`:
```rust
let config_max = Some(3u8);
  if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```

Using `if let` let's us use less boiler plate for how ever it lose exhaustive checking of `match`, choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

We can include `else` as well, the code block will be run in the same case `_` in `match` expression

Example of program that count every coin that is not a quarter using `match`:
```rust
let mut count = 0;
  match coin {
      Coin::Quarter(state) => println!("State quarter from {state:?}!"),
      _ => count += 1,
  }
```

Example of the same program now using `if let` and `else`:
```rust
let mut count = 0;
  if let Coin::Quarter(state) {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
```

### The happy path with `let...else`
Continuing with our example of coins with a UsState value, if we wanted to say something funny depending on how old the state on the quarter was, we might introduce a method on UsState to check the age of a state, like so:
```rust

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}
```

First we might use `if let` to match on a type of coin, introducing state of variable within the body of condition:
```rust

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
```

This get the job done but it push it works to the `if let` statements, we can take advantage of the fact that expressions produce value:
```rust

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

This is a bit annoying since one branch produce value and another one return from the function entirely, To make this common pattern nicer Rust has `let...else` it really similar to `if let` but does not have the `if` branch only `else` branch:
```rust

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

This helps us stay on happy path, without having significantly different control flow the way `if let` did

## gotchas

## links
[[(1-3)Rust-basic]]
[Rust Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
