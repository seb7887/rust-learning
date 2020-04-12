# Rust

- Is a modern systems programming language that is focused on safety, speed, and concurrency
- It accomplishes these goals by being memory-safe without garbage collection

## How to compile

For example:

`rustc --out-dir <out_dir> hello.rs`

This will produce an binary file that can be executed

`./hello`

## Formatted print

Printing is handled by a series of `macros` defined in `std::fmt` some which include:

- `format!`: write formatted text to _String_
- `print!`: write formatted text to the console (`io::stdout`)
- `println!`: same as print! but a newline is appended
- `eprint!`: same but printed to the standard error (`io::stderr`)
- `eprintln!`: same but with a newline

## Debug

The fmt::Debug trait makes this very straightforward. All types can derive (automatically create) the fmt::Debug implementation. This is not true for fmt::Display which must be manually implemented.

## Primitives

- **signed integers**: `i8`, `i16`, `...` `isize` (pointer size)
- **unsigned integers**: same with `u` prefix
- **floating point**: `f32` and `f64`
- **char** unicode values
- **bool** true/false
- **()** empty tuple

- **arrays** [1,2,3]
- **tuples** (1, true)

## Variables

### No mutable

Cannot be changed
`let variable: bool = true`

### Mutable

Can be changed
`let mut variable: bool = true`

## Expressions

- Blocks are expressions too, so they can be assigned to the place expression such as a local variable
- To end with a **semicolon** a block expression is to return a `()`

## Functions

- If the return value is an expression, don't put a semicolon at the end
  `lhs % rhs == 0`
- If the return value is a sentence, put a semicolon at the end
  `return true;`

## Cargo

- Is the official Rust package management tool. It has integration with `crates.io` (the official Rust package registry)

### Create a new Rust binary

`cargo new foo`

### Create a new Rust library

`cargo new --lib foo`

### File structure

```
foo
|- Cargo.toml
|- src
  |- main.rs
```

### Cargo.toml example

```toml
[package]
name = "foo"
version = "0.1.0"
authors = "Sebastian Matias Segura"
build = "build.rs" # build script

[dependencies]
clap = "2.27.1" # from crates.io
rand = { git = "https://github.com/rust-land-nursery/rand" } # from online repo
bar = { path = "../bar" } # from a path in the local filesystem
```

### Cargo commands

- `cargo build`: build
- `cargo run`: build and run
- `cargo test`: run tests

## Traits

- A collection of methods defined for an unknown type: `Self`
