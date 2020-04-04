// Complile with rustc --crate-type=lib library.rs

pub fn public_fn() {
  println!("called seb's `public_fn()`");
}

fn private_fn() {
  println!("Called seb's `private_fn()`");
}

pub fn indirect() {
  private_fn();
}
