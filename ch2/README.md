# Chapter 2. Programming a Guessing Game
- Building a guessing game program
- Learn about `let`, `match`, methods, associated functions, using external crates, and more
- Practice fundamentals

## What Did I Learn

### Variables
- `let` keyword used to create a variable
```rust
    let apples = 5
```
- variables are immutable by default, similar to a `const` (constant) in javascript
- immutable means the value give to the variable won't change once it is set
- `mut`: makes a vaiable mutable 
```rust
    let apples = 5      // immutable (cannot be changed later)
    let mut apples = 5  // mutable (can be changed later)
```
- `::` syntax indicates that `new` is an associated function of `String`
- `::` is similar to the `.` notation in many other languages
- in rust, `::` specfically denotes `Object-has-function` or `Object::function`
- in this case, we are simply defining `guess` to be a new `String` object
```rust
    let mut guess = String::new();
```

### Imports
- can use `std::io` library without doing initial import at top of file
- initial import makes it more explicit before running fn logic, e.g.
```rust
    use std::io;

    io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
```
- requires explicit declaration in the code 
```rust
    std::io::stdin().read_line(&mut guess)
```
- using the explicit call returns an instance of `std::io::stdin`

### Modifiers
`&` modifies variable declaration to create a reference to the var's value
- space saving technique
- turns the variable into a `reference` to that variable's value
- allows multiple parts a program access one data instance
- do not need to copy the data into memory multiple times
- refereces are immutable by default, must use `&mut` before variable to gain mutability
```rust
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
```

### Enums
*enum* data type having many states called a *variant*
- `read_line` returns a `Result` enum with two variants
```rust
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn read_line(&self, buf: &mut String) -> io::Result<usize> {
        self.lock().read_line(buf)
    }
```
- `Result` object encodes error-handling info as *variants*
    - `Ok()`: variant handles success
    - `Err()`: variant handles fail
```rust
    pub enum Result<T, E> {
        /// Contains the success value
        #[lang = "Ok"]
        #[stable(feature = "rust1", since = "1.0.0")]
        Ok(#[stable(feature = "rust1", since = "1.0.0")] T),

        /// Contains the error value
        #[lang = "Err"]
        #[stable(feature = "rust1", since = "1.0.0")]
        Err(#[stable(feature = "rust1", since = "1.0.0")] E),
    }
```
- `Result` has methods you can call, including `expect`
- Not using `expect` on an object that returns a `Result` will compile with warnings
```rust
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!")
```
- `Ordering` is an *enum* with 3 *variants*
    - `Less`: compared value is less than another
    - `Greater`: compared value is greater than another
    - `Equal`: compared value is equal to another
```rust
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
    }
```
### Number Data Types
- Some of rust's `number` types:
    - `i32` 32-bit number
    - `u32` unsigned 32-bit number
    - `i64` 64-bit number

### Library Functions
- No random number functionality in Rust std lib
- `rand` crate can be installed as a *library crate* in contrast to this guessing game program, which is a *binary crate*
```conf
# Cargo.toml
[dependencies]
rand = "0.9.0"
```
- Cargo fetches the latest versions of the external dependencies from the *registry*
- The *registry* is a copy of data from [Crates.io](https://crates.io/)
- Cargo auto detects changes to `Cargo.toml` aand source code when running `cargo build`
- `Cargo.lock` ensures reproducible builds by locking dependency versioning for the project build into whatever is specified in the `Cargo.toml`
- `Cargo.lock` also makes re-running `cargo build` much faster since `cargo` won't need to figuring out the versions again
- `cargo update`: ignores `Cargo.lock` and figures out all the lastest versions that fit your specs in `Cargo.toml`
- `cargo doc --open`: builds docs provided by dependencies locally and open in browser, each crate has documentation with instructinos for use
```sh
cargo update
cargo build
cargo doc --open
```

### Methods & Keywords
- `trim` method on a String eliminates \n or \r\n
- `parse` method on strings converts a string to another type
- `loop` keyword creates an infinite loop
```rust
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /* match the type u32 against the guess to determine if guess is u32 */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
```