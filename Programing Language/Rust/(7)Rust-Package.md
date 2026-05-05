---
id: "learn: Rust-Package(7)"
aliases: []
tags:
  - learn
  - rust
  - package
---

2026-04-28 Init 16:59

# Rust-Package

## what is it
Packages, Crates, and Modules are the way to organized code of Rust in large project including which detail are expose which detail are private and manage name in each scope of program, This collectively call *modules system*, insist of:
  * **Packages**: A cargo feature to build, test, and share creates
  * **Crates**: Tree of modules that produce library or executable
  * **Modules: and use**: Control the organization, scope, and privacy of paths
  * **Paths**: A way of naming an item, such as a struct, function, or module

## how it works
### Packages and Crates
**Crate** is the smallest amount of code that Rust compiler considers at a time even if running with `rustc` instead of `cargo`, event with single file program like simple hello word rustc treat that as a crate. Crate may contain module and modules may be define in other file and get compile with crate.

Crates come with 2 form:
  * *binary crate*: It compile to executable and can run, each must have `main` function that define what happen when program run.
  * *library crate*: Unlike binary crate, this one dos not contain `main` function and not produce executable, they define functionality to be share with multiple projects. 

Crate root is a source file that rustc start form and made up the root module of the crate. 

**Package** must contain at least one crate and can have as many binary crate, but only one library crate

Example of rust package with `cargo`:

```bash
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

After we create package and `ls`, there is `Cargo.toml` file, giving us package. If we open `Cargo.toml` there is no mention of `src/main.rs`, Cargo follow a convention that `src/main.rs` is a root of a binary crate with same name as package, same go for the `src/lib.rs` that contain library crate that is a root of library crate and `src/lib.rs` is the root, Cargo passes the crate file to rustc to build library or binary. 

### Control scope privacy with modules
#### Modules cheat sheet
* Starting from crate root: Usually compiler look for `main.rs` or `lib.rs` for code to compile

* Declaring modules: Modules can be declare with `mod` for example `mod garden;`. The compiler will look for module code in theses place:
  - Inline, within curly brackets that replace `;` follow `garden`
  - In the file `src/garden.rs`
  - In the file `src/garden/mod.rs`

* Declaring submodules: For example in `src/garden.rs`.The compiler will look for the submodule’s code within the directory named for the parent module in these places:
  - Inline, directly following mod vegetables, within curly brackets instead of the semicolon
  - In the file `src/garden/vegetables.rs`
  - In the file `src/garden/vegetables/mod.rs`

* Paths to code in the modules: Once a module is part of your crate, you can refer to code in that module from anywhere else in the same crate as long as privacy allow, for example:
  an `Asparagus` is a type in the garden vegetable module would be found at `crate::garden::vegetable::Asparagus`

* Private vs. Public: Code in within a module is private from its parent by default. To make a module public use `public mod`, to make items in the module public as well use `pub` before declare ration.

* The `use` keyword: it use to shortcut long path For example: `use crate::garden::vegetable::Asparagus` than after this we can just write `Asparagus`

To illustrate this rule Imagine a crate call `backyard` that has this structure:

  backyard
  ├── Cargo.lock
  ├── Cargo.toml
  └── src
      ├── garden
      │   └── vegetables.rs
      ├── garden.rs
      └── main.rs

File: src/main.rs

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

`pub mod garden;` line tell compiler to include the code found at `src/garden.rs`

File: garden.rs
```rust
pub mod vegetable;
```

Here this tell compiler that code at `src/garden/vegetable.rs` is include too

File: vegetable.rs
```rust
#[derive(Debug)]
pub struct Asparagus {}
```

#### Grouping relate code in module
Let's create a library name `restaurant` that represent functionality of the front of restaurant hosting table, serving etc. Create a new library with.

```bash
cargo new restaurant --lib
```

File name: `src/lib.rs` to focus on learning module we will leave implementation of a function empty here.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

Module tree structure:

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

Entire module that root under the implicit of create that each submodules like hosting and serving nest under front_of_house which is parent. This works kinda like filesystem directory tree

### Paths for Referring to an Item in the Module Tree
Take a look at the Example:
```rust

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

The Absolute path start from the crate root, if it the same crate we use literal `crate` while relative path start from the modules name in the same file that module is in that why it call relative,

How ever the code above will not compile because `hosting` is a private modules and in Rust all items(functions, methods, structs, enums, modules, and constants) are private by default, to actually access `add_to_waitlist` we need to use `pub`

Compiler output:
```

   Compiling rust-sandbox v0.1.0 (/home/eri/rust-sandbox)
error[E0603]: module `hosting` is private
 --> src/main.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
  |                            |
  |                            private module
  |
note: the module `hosting` is defined here
 --> src/main.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/main.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
   |                     |
   |                     private module
   |
note: the module `hosting` is defined here
  --> src/main.rs:2:5
   |
 2 |     mod hosting {
   |     ^^^^^^^^^^^

error[E0601]: `main` function not found in crate `rust_sandbox`
  --> src/main.rs:13:2
   |
13 | }
   |  ^ consider adding a `main` function to `src/main.rs`

Some errors have detailed explanations: E0601, E0603.
For more information about an error, try `rustc --explain E0601`.
error: could not compile `rust-sandbox` (bin "rust-sandbox") due to 3 previous errors
```

### Exposing path with `pub` keyword

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}
```

Now the `mod hosting` is public but however this will still not compile because even with `hosting` being public it mean let's code in ancestor module refer to it, not access the inner code because module is just a container to actually access `add_to_waitlist` we also need to use `pub` there

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

Now the code is valid and will compile! Because both `hosting` and `add_to_waitlist` are now public

**Best practice for package with binary and library**
Because a package can have both binary crate and library crate so with `src/main.rs` and `src/lib.rs`, the module tree should define in `src/lib.rs` and used in the binary crate by starting path with the name of the package the binary crate become user of the library crate just like like the external crate can use library crate, this API design let's programmer become both user and client.

### Starting Relative path with `super`
`super` use to create path that begin at the parent module instead of current module or crate root, like using `..` syntax for the filesystem path to go to parent directly

Exmaple of using super Filename: `src/lib.rs`:
```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

In function `fix_incorrect_order` we use `super` that refer to `deliver_order` that is in parent module. Therefore, we used super so that we’ll have fewer places to update code in the future if this code gets moved to a different module.

### Making Structs and Enums Public
When we use `pub` before declaration of struct it will only make struct itself public not it field, if we want any field in struct to be public as well we also need to use `pub` annotation for each field, Unlike Enums, if we use `pub` before declaration all the enums field will be public by default

Example:
```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}
```

In `Breakfast` we also annotate `toast` with `pub` to make ti public as well while all field in `Appetizer` are public by default.

Also note that since `seasonal_fruit` is a private field in order for the `eat_at_restaurant` to create struct instant, it need to provide associate factory functions to in this example `summer`, if we uncomment last line it will not compile because we could not set private value of `seasonal_fruit` field.

Rust do this because enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public. Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.

### Bringing Paths into Scope with the `use` Keyword
Take a look at the example:
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

By using `use` we now bring `front_of_house` into `eat_at_restaurant` scope, and after that if we want to use `add_to_waitlist` we just need to specify the shorter path which is more convenience than have to write `crate::front_of_house::hosting;` every time

However `use` only make path valid in particular scope if we move `eat_at_restaurant` into other scope the code will not compile:
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
  pub fn eat_at_restaurant() { 
      hosting::add_to_waitlist(); // This is in the different scope
  }
}
```

To fix this we need to move `use crate::front_of_house::hosting;` into the same scope with `eat_at_restaurant` or use `super::hosting::add_to_waitlist();` instead 

#### Creating Idiomatic `use` Paths

In the last example it also valid to use `add_to_waitlist` instead of `hosting::add_to_waitlist` but if we just use the function name it will not clear where is that function define, is it local or come from another module so it's a convention to also include parent module with the function itself

On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path:

```rust
use std::collections::HashMap; // full path form the std crate

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

Only exception to this idiom is when bringing 2 item with the same name into the scope with `use` statement because Rust doesn't allow that, the work around is specify them with parent module:

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result { // std::fmt is type aliased to specific type define as type Result = Result<(), Error>;
    // --snip--
}

fn function2() -> io::Result<()> { // while std::io type is generic <T> so we define it as unit () on return here. 
   // --snip--
}
```

In the example above both result have parent module to distinguishes them from each other.

#### Providing new name with `as` Keyword 

There also another solution to prevent name conflict in the same scope with new specify local name with `as` keyword:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

Here `function2` return `IoResult` instead of just `Result`. Both solution of provide new name with `as` keyword and prefix with parent module are considered to be idiomatic so the choice is up to the programmer.

#### Re-exporting Names with `pub use`
To enable code outside that scope to refer to that name as if it had been defined in that scope, we can combine pub and use:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Without this change, external code would have to call `add_to_waitlist` function by using path `restaurant::hosting::add_to_waitlist()` which also required `front_of_house` to be prefix with `pub`, but now this `pub use` re-exported the `hosting` module form root module, external code can use `restaurant::hosting::add_to_waitlist()` instead.

Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. For example, in this restaurant metaphor, the people running the restaurant think about “front of house” and “back of house.” But customers visiting a restaurant probably won’t think about the parts of the restaurant in those terms. With pub use, we can write our code with one structure but expose a different structure. Doing so makes our library well organized for programmers working on the library and programmers calling the library.

#### Using External Packages
In Chapter 2 we used an external package call `rand` to get the random numbers. To use `rand` in our project, we add `rand = "0.8.5"` in `Cargo.toml` file, adding `rand` as dependency that tells Cargo to download the `rand` and other dependencies from `crate.io`

Then Rust use `use` keyword to bring package into the scope:

```rust
use rand::Rng; // Bringing rand into current scope

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

Note that `std` is also external package but already shipped with Rust, it don't need to list in `Cargo.toml` file:

```rust
use std::collections::HashMap;
```

This is a literal path of `std`, the standard library crate

#### Using Nested Paths to Clean Up use Lists
Instead of using multiple `use` line, Rust can simplify them in to one line with Nested paths if they share the common path:
```rust
// --snip--
use std::cmp::Ordering;
use std::io;

// can be simplify to nested path as:
use std::{cmp::Ordering, io};
```

We can use a nested path at any level in a path, which is useful when combining two use statements that share a subpath:
Another Example:
```rust
// --snip--
use std::io;
use std::io::Write;

// can be simplify to nested path as:
use std::io::{self,Write};
```

Here we use `self` to refer to `std::io`

#### Importing Items with Glob Operator
If we want to bring all public items define in the path into scope:
```rust

use std::collections::*;
```

This `use` statement brings all public items defined in `std::collections` into the current scope. Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined. Additionally, if the dependency changes its definitions, what you’ve imported changes as well, which may lead to compiler errors when you upgrade the dependency if the dependency adds a definition with the same name as a definition of yours in the same scope

### Separating Modules into Different Files
To separate module into different file:

Filename: `src/lib.rs`
```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

First we leave only declaration of `front_of_house` then move curly brackets `{}` and it implementation into different file:

Filename: `src/front_of_house.rs`

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

Now if we also want to move `hosting` into different file, it a little bit different because it is submodules of `front_of_house`:

Filename: `src/front_of_house.rs`

```rust
pub mod hosting;
```

Leave only declaration than:

Filename: `src/front_of_house/hosting.rs`

```rust
pub fn add_to_waitlist() {}
```

Function definition in `hosting.rs`

**Alternative file Paths**

So far we’ve covered the most idiomatic file paths the Rust compiler uses, but Rust also supports an older style of file path. For a module named front_of_house declared in the crate root, the compiler will look for the module’s code in:

* `src/front_of_house.rs` (what we covered)
* `src/front_of_house/mod.rs` (older style, still supported path)

For a module named hosting that is a submodule of front_of_house, the compiler will look for the module’s code in:

* `src/front_of_house/hosting.rs` (what we covered)
* `src/front_of_house/hosting/mod.rs` (older style, still supported path)

If you use both styles for the same module, you’ll get a compiler error. Using a mix of both styles for different modules in the same project is allowed but might be confusing for people navigating your project.

The main downside to the style that uses files named mod.rs is that your project can end up with many files named mod.rs, which can get confusing when you have them open in your editor at the same time.

**Summary**

Rust let programmer split package into multiple crate and multiple crate into multiple modules so we can refer to another module from different modules by specify Relative path or Absolute path with `use` keyword, module code body is private by default so to make it public we use `pub` keyword. 

## gotchas

## links
[Rust Package](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html#alternate-file-paths)
[[(1-3)Rust-basic]]
