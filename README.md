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
