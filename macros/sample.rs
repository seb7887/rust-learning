macro_rules! say_hello {
  // () indicated that the macro takes no arguments
  () => {
    println!("Hello!");
  };
}

fn main() {
  say_hello!()
}