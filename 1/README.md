# Chapter 1. Getting Started
- Install Rust
- Write `Hello, World!` program
- Use `cargo`

## Section 1.3. Hello, Cargo!

- Cargo is Rust’s build system and package manager

### Cargo Basics

1. `cargo new`: lets you create a project.
2. `cargo build`: lets you build a project.
3. `cargo run`: lets you  build and run a project in one step.
4. `cargo check` lets you check for errors by building a project without producing a binary.
5. `target/debug`: Cargo saves the result of a build in the `target/debug` folder instead of the same directory as our code.

### Fun Facts
- Cargo commands are the same no matter which operating system you’re working on!

### Advanced Cargo
1. `cargo build --release`: lets you compile your project with optimizations when its ready for release
2. `target/release`: `cargo build --release` creates an executable in `target/release` instead of `target/debug`

## What Did I Learn
- The basics of a rust function
- Facts about the language and tooling
- How to use cargo and its advantages