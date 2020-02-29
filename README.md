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
