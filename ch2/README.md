# Chapter 2. Programming a Guessing Game
- Building a guessing game program
- Learn about `let`, `match`, methods, associated functions, using external crates, and more
- Practice fundamentals

## What Did I Learn
- `let`: used to create a variable
- e.g.
```rust
    let apples = 5
```
- variables are immutable by default, similar to a `const` (constant) in javascript
- immutable means the value give to the variable won't change once it is set
- `mut`: makes a vaiable mutable
- e.g. 
```rust
    let apples = 5      // immutable (cannot be changed later)
    let mut apples = 5  // mutable (can be changed later)
```
- `::` syntax indicates that `new` is an associated function of `String`
- i.e. `::` is similar to the `.` notation in many other languages, however, in rust, it is specfically to denote that an Object-has-function or `Object::function`
- can still use `std::io` library without doing initial import at top of file
- e.g. using `std::io::stdin().read_line(&mut guess)` returns an instance of `std::io::Stdin`
- ampersand `&` indicates that the argument in the method call is a `reference`
- `&` lets multiple parts of the same code access one piece of data without needing to copy the data into memory multiple times
- refereces are also immutable by default thus why we use `&mut guess`
- `read_line` returns a `Result` value, which is an enumeration *enum*
- an *enum* is a data type that can be in 1 of many states, called a *variant*
- `Result` *variants* encode error-handling info:
    - `Ok`: operation was successful
    - `Err`: operation failed
- `Result` has methods you can call, including `expect`
- Not using `expect` on an object that returns a `Result` will compile with warnings
- No random number functionality in Rust std lib
- `rand` crate can be installed as a *library crate* in contrast to this guessing game program, which is a *binary crate*
- Cargo fetches the latest versions of the external dependencies from the *registry*
- The *registry* is a copy of data from [Crates.io](https://crates.io/)
- Cargo auto detects changes to `Cargo.toml` aand source code when running `cargo build`
- `Cargo.lock` ensures reproducible builds by locking dependency versioning for the project build into whatever is specified in the `Cargo.toml`
- `Cargo.lock` also makes re-running `cargo build` much faster since `cargo` won't need to figuring out the versions again
- `cargo update`: ignores `Cargo.lock` and figures out all the lastest versions that fit your specs in `Cargo.toml`
- `cargo doc --open`: builds docs provided by dependencies locally and open in browser, each crate has documentation with instructinos for use
- `Ordering` type (via `use std::cmp::Ordering`) is an *enum* with *variants* `Less`, `Greater` and `Equal`
- Some of rust's `number` types:
    - `i32`: 32-bit number
    - `u32`: unsigned 32-bit number
    - `i64`: 64-bit number
- Rust allows us to shadow the value of a previously definred variable with a new variable of the same name with a diff value in the same program
```rust

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!")
```
- The `trim` method on a String eliminates \n or \r\n
- The `parse` method on strings converts a string to another type
- `loop` keyword creates an infinite loop