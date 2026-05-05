---
id: "learn: Rust-struct"
aliases: []
tags:
  - learn
  - rust
---

2026-04-21 Init 03:02

## what is it
A structs, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

## how it works
Example of struct definition:

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

Creating an instance of the struct and accessing its fields with dot notation:

```rust
fn main() {
    let mut user1 = User { // mut makes the instance mutable so we can change its fields later
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com"); // accessing and modifying a field using dot notation
}
```

Function that returns an instance of the struct can use a field init shorthand syntax when the parameter names are the same as the struct field names:

```rust
fn build_user(email: String, username: String) -> User {
  user {
    email, // same as email: email
    username, // same as username: username
    active: true,
    sign_in_count: 1,
  }
}
```

Creating a new instance of a struct from another instance with struct update syntax:

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // use the remaining fields from user1
    };
}
```

Structs is still follow the move semantics, so when we assign an instance of a struct to another variable or pass it to a function, the ownership of the data is moved rather than copied. If we want to create a new instance that has the same values as an existing instance, we can use the `..` syntax to specify that we want to use the remaining fields from the existing instance.

However, `user1` can still be used if `user2` has new values for the fields that are not `Copy` types (email, username), because the ownership of those fields is moved to `user2`, but the ownership of the fields that are `Copy` types is not moved, so `user1` can still be used.

### Tuple Structs
Tuple structs are a kind of struct that don't have named fields, but instead have unnamed fields that are accessed by their position in the tuple. They are useful when you want to create a struct that has a small number of fields and you don't need to name them.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

Even Color and Point have the same types for their fields, they are still different types because they are defined as different structs. This can be useful for type safety, as it prevents us from accidentally using a Color where a Point is expected, or vice versa.

Tuple structs can be deconstructed into their individual fields and access individual fields with dot notation and the index of the field:

```rust
let Point(x, y, z) = origin; // deconstructing a tuple struct into its individual fields
oringin.0; // accessing the first field of the Point struct
```

### Unit like Structs
To create a struct that doesn't have any fields, we can use unit-like structs. They are useful when you want to implement a trait on some type but don't have any data that you want to store in the type.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

**Ownership of structs date**
In previous example, `user` structs use `String` rather than `&str` This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid but it also possible to store references in a struct, but we need to specify the lifetime of the references to ensure that they are valid for as long as the struct is valid. This is done using **lifetime** annotations.

## example

```rust
// calulate the area of a rectangle using a struct to hold the width and height
struct Retangle {
    width: u32,
    height: u32,
}

fn main() {
    let reactangle1 = Retangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&reactangle1)
    );
}

// Passing reference to the rectangle struct to the area function so that we can calculate the area without taking ownership of the rectangle struct
fn area(rectangle: &Retangle) -> u32 {
    rectangle.width * rectangle.height
}
```

### Methods
Methods are functions that are defined within the context of a struct and have access to the data of the struct. They are defined using the `impl` keyword and can be called using dot notation on an instance of the struct:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
  // Reference to self allows us to use the instance of the struct that the method is being called on, and we can access the fields of the struct using dot notation within the method and it will not take ownership of the struct instance, so we can call multiple methods on the same instance without worrying about ownership issues.
    fn width(&self) -> bool {
        self.width > 0
    }

  // Multiple parameters in a method, we can use references to other instances of the same struct to compare their fields and return a boolean value indicating whether the current instance can hold the other instance based on their width and height.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

   if rect1.width() {
    // with parentheses, we call the method `width` and get a boolean value indicating whether the width of the rectangle is greater than 0. If it is, we print a message that includes the width of the rectangle using dot notation to access the `width` field of the `rect1` instance.
        println!("The rectangle has a nonzero width {}; it is {}", rect1.width(), rect1.width);
    }

    if rect1.can_hold(&rect2) {
        println!("rect1 can hold rect2");
    } else {
        println!("rect1 cannot hold rect2");
    }
}
```

#### Associated Functions
Associated fuctions are functions that are defined within the context of a struct but don't have access to the data of the struct. They are defined using the `impl` keyword and can be called using the double colon syntax `::` on the struct itself rather than an instance of the struct:

```rust
impl Rectangle {
  fn square(size: u32) -> Rectangle {
    self {
      width: size,
      height: size,
    }
  }
}

fn main() {
  let sq = Rectangle::square(3); // calling the associated function `square` on the `Rectangle` struct to create a new instance of `Rectangle` that has equal width and height based on the size parameter passed to the function
}
```

#### Multiple `impl` blocks
```rust

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

There is no reason to separate the `impl` blocks in this way, but it is allowed in Rust. You can have as many `impl` blocks as you want for a given struct, and they can be defined in different parts of your code. This can be useful for organizing your code and keeping related methods together.

### Summary
Structs are a powerful way to create custom data types in Rust that can hold multiple related values. They can be defined with named fields, tuple fields, or no fields at all. Methods can be defined on structs to provide functionality that operates on the data of the struct, and associated functions can be defined to provide functionality that is related to the struct but doesn't operate on its data. Structs follow Rust's ownership rules, so you need to be mindful of how you use them in your code.

## gotchas
- Partially move struct:

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--
    let mut user1 = User { // mut makes the instance mutable so we can change its fields later
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        username: user1.username.clone(), // need .clone() to explicitly deep copy username form user1, so this way user1 is still valid
        email: String::from("another@example.com"),
        ..user1 // use the remaining fields from user1
    };
}
```

Even thou `user1` and `user2` does not share the email but just `username`, since `username` is a string that have move trait so it now move to `user2` and `user1` is now incomplete(missing username) and considered to be invalid by Rust to prevent access to memory location that is moved

- `self` VS `Self`: self is refers to the instance of the struct that the method is being called on, while Self refers to the type of the struct itself. In other words, self is a reference to the current instance of the struct, while Self is a reference to the struct type.

Example:

```rust
impl Rectangle {
  fn new(width: u32, height: u32) -> Self { // Self refers to the Rectangle struct type
    Self { // Self is used to create a new instance of the Rectangle struct
      width,
      height,
    }
  }

  fn area(&self) -> u32 { // self refers to the instance of the Rectangle struct that the method is being called on
    self.width * self.height // using self to access the fields of the struct instance
  }
}
```

## links
[[(1-3)Rust-basic]]

