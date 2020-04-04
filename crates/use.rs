// Run with rustc use.rs --extern library=liblibrary.rlib && ./use
extern crate library;

fn main() {
  library::public_fn();

  library::indirect();
}
