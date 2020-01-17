#[derive(Debug)] 
struct Person<'a> {
  name: &'a str,
  age: u8
}

fn main() {
  let name = "Peter";
  let age = 32;
  let peter = Person { name, age };

  // Pretty print
  println!("{:#?}", peter);
}