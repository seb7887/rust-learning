// const: unchangeable (common case)
// static: a possibly mutable variable with 'static lifetime
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
  // Accessing constant in some function
  n > THRESHOLD
}

fn main() {
  let n = 16;

  println!("This is {}", LANGUAGE);
  println!("The threshold is {}", THRESHOLD);
  println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}