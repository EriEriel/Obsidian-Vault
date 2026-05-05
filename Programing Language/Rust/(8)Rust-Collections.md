---
id: "learn: Rust-collections(8)"
aliases: []
tags:
  - learn
  - "[ ] rust"
  - collections
---

2026-05-02 Init 00:39

# Common Collections

## what is it
Unlike built in array and tuple, this common collections are store on heap and don't know the size at compile time, can grow or shrink in size, each kind of collection has different capabilities and costs:
  * **Vector** allow to store a variable of number next to each other.
  * **String** a collections of characters.
  * **Hash map** allow to give associate key to the value. It’s a particular implementation of the more general data structure called a map. 

## how it works
### Vector

#### Storing list of value with Vectors
**Vector** `Vec<T>` allow to store same type of value next to each other on the memory to create vector, call `Vec::new`:

```rust
let v: Vec<i32> = Vec::new();
```

`Vec<T>` can hold any type of data and implement with generic so, since the example above doesn't provide value so it need to type annotate with `<i32>`

`Vec<T>` with initial values will refer to it type of value that it store so it rarely need type annotation, Rust also provide vector macro with `vec!` to create and hold value that programmer give it:

```rust
let v = vec![1, 2, 3];
```

This vector hold type `i32` sine it default number type in Rust, and Rust can refer to `v` type as `Vec<i32>` and type annotation isn't necessary

### Updating Vector
To update the vector it need to be prefix with `mut` to be mutable and than use `push` method:

```rust
let v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
v.push(9);
```

Since all value are type of `i32`, and Rust infers this from the data, so it don't need `Vec<i32>` annotation

#### Reading Element of Vector
To read an element in vector Rust provide 2 method index and, `get` method as in the example:

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

Element can be read by index using `&` reference and `[]` giving reference to index value while `get` method pass index into argument and return with `Option<&T>` that can use with `match`

Rust provide this 2 ways to reference to an element so programmer can choose how program will behave, as in example:

```rust
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

The `&v[100]` will cause program to panic since it out of vector range while `v.get(100)` return `None` with out panicking.

When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules (covered in Chapter 4) to ensure that this reference and any other references to the contents of the vector remain valid:

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {first}");
```

This will not compile since it has both immutable borrow and mutable borrow in the same scope, The reason while Rust prevent this even programmer access the different index is because with `push` Rust need to allocate vector to new memory block since it increase in size and `first` might pointing to deallocate memory. The borrowing rules prevent programs from ending up in that situation.

The comprehensive detail of Rust vector implementation:
[The Rustonomicon- Implement Vector from scratch](https://doc.rust-lang.org/nomicon/vec/vec.html)

#### Iterating over value in Vector
To iterating over vector, we can use `for` loop:

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}
```

Then to iterate over mutable references to each element in a mutable vector:

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

Note that in `mut` references we need to use `*` dereference because `println!` macro already do implicit `deref coercion` while for mutable we need to be explicit since `i += 50;` mean we try to add `50` to memory address which is impossible so it need `*` dereference.

Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker’s rules. The reference to the vector that the for loop holds prevents simultaneous modification of the whole vector.

#### Using Enum to store Multiple Types
Since vector can only contian same type of value, in order to store multiple type of value we can use enum to create vector that holds enum:

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

Rust needs to know what types will be in the vector at compile time so that it knows exactly how much memory on the heap will be needed to store each element. We must also be explicit about what types are allowed in this vector. Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled

If you don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a trait object

The [The API documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html) all of the many useful methods defined on `Vec<T>` by the standard library.

#### Dropping a Vector Drop it Elements
When vector is freed when goes out of scope all elements of vector will be drop for good:
```rust

{
    let v = vec![1, 2, 3, 4];

    // do stuff with v
}// <- v goes out of scope and is freed here
```

Everything will be clean up when drop. The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.

### UTF-8 Encoded Text with Strings 

#### Defining Strings
Rust core language has only one type of string, which is string slice `str` that is usually seen in the borrowed form `&str`. String literals, for example, are stored in the program’s binary and are therefore string slices.

The `String` type that provide by Rust's standard library rather coded into core language, is growable, mutable, owned, UTF-8 encoded string type, both string slice `&str` and `String` are UTF-8 encoded.

#### Creating new String
`String` is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities, therefore many of `Vec<T>` operation is also available for `String` as well:

```rust
let s = String::new(); // can use new() the same ways as vector
```

Using `to_string` method that available on any type that implement `Display` trait, as string literal do:

```rust
let data = "initial_content";

let s = data.to_string();

// can also work directly on literal string
let s = "initial_content".to_string();
```

Use `String::from()` function to create string same ways as vector:
```rust

let s = String::from("initial_content");
```

Since Rust string is UTF-8 encoded so:

```rust
let emoji = String::from("🦀");
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שלום");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```

All of these are valid Rust string value.

#### Updating a String

**Appending with `push_str` or `push`**:

```rust
let mut s = String::from("foo");
s.push_str("bar");

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2}");
```

Since `push_str` take string slices `&str` so it does not take ownership of `s2`

Note that `&str` is a fat pointer that store address and length of the string and live on stack while actual text is on the heap
Example:
```rust
let s = "hi";
let s2 = String::from("hi"); 
```

| Feature | Stack | Heap | Binary(Satatic) |
| --------------- | --------------- | --------------- | --------------- |
| What'there | Local variables, Fat pointer | Growing data, long live buffer | Hardcoded text, program instructions |
| `let s = "hi";` | The `&str` fat pointer | (Nothing) | The actual bytes `"hi"`. |
| `let s2 = String::from("hi");` | The `String` metadata | The actual bytes `"hi"` | (Nothing) |

`push` method take single characters as argument:

```rust
let mut s = String::from("lo");
s.push('l');
```

As the result `s` will contain `lol`

**Concatenating with `+` or `format!`**

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

The reason that `s1` is no longer valid and why `s2` need to use `&` come down to add `+` definition:

```rust
fn add(self, s: &str) -> String {
```

Since it take `s1` as self so `s1` move to add function than accept argument of `&s2` which make `s2` still valid afterward. It literally take owner ship of `s1` than append with copy of `s2` and than bind to `s3`, This implementation is more efficient than just to copy both `s1` and `s2`.

However Concatenating multiple strings, the behavior of `+` operation gets unwieldy:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```

This is a bit difficult to see what is going on so Rust provide `format!` macro:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```

Now `s` will be `tic-tac-toe`, `farmat!` work like `println!` but instead of printing output to the screen, it return `String` with the contents. `format!` macro uses references so that this call doesn’t take ownership of any of its parameters.

#### Indexing into String
Since `String` in Rust is a wrapper of `Vec<u8>`, Rust string don't support indexing because:

```rust
let hello = String::from("Halo");
```

This is case `len` will be `4`, which mean `halo` store 4 bytes long, each character take 1 bytes when encoded in UTF-8 

```rust
let hello = String::from("Здравствуйте");
```

This might look like it's 12 string long but in fact for Rust it's 24 that is how much to encode `"Здравствуйте"` in UTF-8, because each Unicode of scalar value in this string take 2 bytes of storage. Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value.

To demonstrate, Consider this code to be valid:

```rust
let hello = "Здравствуйте";
let answer = &hello[0];
```

The expected answer would be `З` but in fact it is `208` and `208` is not valid character on it own its has to combine with second index which is `151` and returning `208` is not what user would expected. 

The answer, then, is that to avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

**Bytes, Scalar Values, and Grapheme Clusters**
Another point about UTF-8 is that there are actual three relevant ways to look at the string from Rust's perspective
Example of Hindi word  “नमस् ते" written in the Devanagari script, it store in the vector of `u8` and the values look like this:

```rust
// vector of u8
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
```

The vector has 18 bytes, that how computer ultimately store the data. Now let's look at Unicode scalar values, which are Rust `char` type, those bytes will look like:

```rust
['न', 'म', 'स', '्', 'त', 'े']
```

There are six char values here, but the fourth and sixth are not letters: They’re diacritics that don’t make sense on their own. Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:

```rust
["न", "म", "स्", "ते"]
```

A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

### Slicing Strings
Rather than indexing into string using `[]` with single number, We can use `[]` with range to create slice that contain particular bytes:

```rust
let hello = "Здравствуйте";
let s = &hello[0..4];
```

Since each character here is 2 bytes long so `s` will be `Зд`, if we only slice particular bytes of character like `[0..1]`, Rust would panic at runtime in the same way as if an invalid index were accessed in a vector:

```
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/collections`

thread 'main' panicked at src/main.rs:4:19:
byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

#### Iterating over Strings
The best way to operate on piece of string is to be explicit about what we want, characters or bytes, we can use `chars()` method for Unicode scalar values and `bytes()` for returning raw byte:

```rust
for c in "Зд".chars() {
    println!("{c}");
}

// Output:
// З
// Д

for b in "Зд".bytes() {
    println!("{b}");
}

// Output:
// 208
// 151
// 208
// 180
```

#### Handling the Complexities of Strings
To summarize, strings are complicated. Different programming languages make different choices about how to present this complexity to the programmer. Rust has chosen to make the correct handling of String data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data up front. This trade-off exposes more of the complexity of strings than is apparent in other programming languages, but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.

The good news is that the standard library offers a lot of functionality built off the String and &str types to help handle these complex situations correctly. Be sure to check out the documentation for useful methods like contains for searching in a string and replace for substituting parts of a string with another string.

### Hash map
`Hashmap<K, V>` is a collections of homogeneous type of value which mean all keys must be the same type so to values:

#### Creating a New Hash map
Unlike `Vector` Hash map has less support from standard library and one of less often use compare to vector and string so need `use` to call from stdlib

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Using a `new()` methods as same as vector or string and then `insert()` to fill will keys and values.

#### Accessing value in the Hash Map

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

Accessing with `score` with `get` method, since `get` return `Option<&V>` if there no value for that hash map it will return `None`. This program handle the `Opttion` with `copied()` so it will get `Option<i32>` rather than `Option<&i32>`, Then use `unwrap_or(0)` to set zero if `scores` does not have value for a key to exhaustive the `Option`.

Implementation of `unwrap_or`:

```rust
impl<T> Option<T> {
    pub fn unwrap_or(self, default: T) -> T { // return the same type
        match self {
            Some(x) => x, // if x is i32
            None => default, // than default is also i32
        }
    }
}
```

**Iterating Hash Map**
Using `for` loop similar to vector:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{key}: {value}");
}
```

Output: 
  Yellow: 50
  Blue: 10

#### Managing ownership in Hash Map
Hash Map follow copy and move rule of ownership, types that have have copy trait like `i32` does get copy but Owned type like `String` move into Hash map:

```rust
use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{}, {}", field_name, field_value);
}
```

This will not compile since both `field_name` and `field_value` move into hash map so try using it after is invalid.

#### Updating a Hash Map
To update an hash map there is multiple way to do it base on what programmer want to archive, totally overwrite old values, adding Key and value only if a key isn't present or updating value based on old value.

##### overwrite a value

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{scores:?}");
```

The output will be `{"Blue": 25}` since with `insert` method it totally overwrite the old value.

##### Adding Key and Value only if a key isn't present

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{scores:?}");
```

The output will be `{"Yellow": 50, "Blue": 10}`, The `entry` API return `Entry` enum, represent if value might or might not exist, and `or_insert` method on `Entry` return `&mut T` for the corresponding `Entry` key if key exist, and if not insert parameter as new value for this key than return `&mut T` to the new value.

##### Updating value base on old value

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{map:?}");
```

The output will be `{"world": 2, "hello": 1, "wonderful": 1}`, The `split_whitespace` method returns an iterator over subslices, separated by whitespace, of the value in `text`. Than `or_insert` put 0 as initial value since `or_insert` return `&mut T` so in order to use it with `+=` operation it need to dereference first with `*`. The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

#### Hashing Functions
By default, HashMap uses a hashing function called SipHash that can provide resistance to denial-of-service (DoS) attacks involving hash tables. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it

### Summary
Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data.

## example
Exercises:

 1. Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

```rust
fn median(mut arr: Vec<i32>) -> i32 {
    if arr.len() <= 1 {
    return arr[0];
    }

    arr.sort();
    arr[arr.len() / 2]
}
```

2. Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!

```rust
fn piglatin(word: String) -> String {
    let first_char = &word[..1];
    let rest = &word[1..];

    if matches!(first_char, "a" | "e" | "i" | "o" | "u") {
        format!("{word}-hay")
    } else {
        format!("{rest}-{first_char}ay")
    }
}
```

3. Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then, let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

```rust
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("Company Directory. Commands:");
    println!("  add [Name] to [Dept]");
    println!("  remove [Name] from [Dept]");
    println!("  list [Dept] | list all");
    println!("  depts | quit\n");

    loop {
        print!("➜ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match parse_command(input) {
            Command::Add { name, dept } => {
                let employees = company.entry(dept.clone()).or_insert_with(Vec::new);
                if employees.contains(&name) {
                    println!("error: {} is already in {}.", name, dept);
                } else {
                    employees.push(name.clone());
                    println!("ok: added {} to {}.", name, dept);
                }
            }
            Command::Remove { name, dept } => {
                match company.get_mut(&dept) {
                    None => println!("error: department \"{}\" does not exist.", dept),
                    Some(employees) => {
                        if let Some(pos) = employees.iter().position(|e| e == &name) {
                            employees.remove(pos);
                            if employees.is_empty() {
                                company.remove(&dept);
                            }
                            println!("ok: removed {} from {}.", name, dept);
                        } else {
                            println!("error: {} not found in {}.", name, dept);
                        }
                    }
                }
            }
            Command::ListDept(dept) => {
                match company.get(&dept) {
                    None => println!("error: department \"{}\" not found.", dept),
                    Some(employees) => {
                        let mut sorted = employees.clone();
                        sorted.sort();
                        println!("\n[ {} ]", dept);
                        for name in &sorted {
                            println!("  {}", name);
                        }
                        println!();
                    }
                }
            }
            Command::ListAll => {
                if company.is_empty() {
                    println!("no employees yet.");
                } else {
                    let mut depts: Vec<&String> = company.keys().collect();
                    depts.sort();
                    println!();
                    for dept in depts {
                        let mut sorted = company[dept].clone();
                        sorted.sort();
                        println!("[ {} ]", dept);
                        for name in &sorted {
                            println!("  {}", name);
                        }
                        println!();
                    }
                }
            }
            Command::Depts => {
                let mut depts: Vec<&String> = company.keys().collect();
                depts.sort();
                if depts.is_empty() {
                    println!("no departments yet.");
                } else {
                    for dept in depts {
                        println!("  {} ({} employees)", dept, company[dept].len());
                    }
                }
            }
            Command::Quit => break,
            Command::Unknown => println!("unknown command. try: add [Name] to [Dept]"),
        }
    }
}

enum Command {
    Add { name: String, dept: String },
    Remove { name: String, dept: String },
    ListDept(String),
    ListAll,
    Depts,
    Quit,
    Unknown,
}

fn parse_command(input: &str) -> Command {
    let lower = input.to_lowercase();

    if let Some(rest) = lower.strip_prefix("add ") {
        if let Some((name_part, dept_part)) = rest.split_once(" to ") {
            let name = name_part.trim().to_string();
            let dept = dept_part.trim().to_string();
            // Preserve original casing from input
            let offset = "add ".len();
            let orig = &input[offset..];
            if let Some((n, d)) = orig.split_once(" to ") {
                return Command::Add {
                    name: n.trim().to_string(),
                    dept: d.trim().to_string(),
                };
            }
            return Command::Add { name, dept };
        }
    }

    if let Some(rest) = lower.strip_prefix("remove ") {
        if let Some((name_part, dept_part)) = rest.split_once(" from ") {
            let offset = "remove ".len();
            let orig = &input[offset..];
            if let Some((n, d)) = orig.split_once(" from ") {
                return Command::Remove {
                    name: n.trim().to_string(),
                    dept: d.trim().to_string(),
                };
            }
            let _ = (name_part, dept_part);
        }
    }

    if let Some(rest) = input.strip_prefix("list ") {
        let target = rest.trim();
        if target.eq_ignore_ascii_case("all") {
            return Command::ListAll;
        }
        return Command::ListDept(target.to_string());
    }

    if lower == "depts"  { return Command::Depts; }
    if lower == "quit" || lower == "exit" { return Command::Quit; }

    Command::Unknown
}
```

## gotchas

## links
[Rust book-Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
[Rust std::collections](https://doc.rust-lang.org/std/collections/index.html)
[[(1-3)Rust-basic]]
