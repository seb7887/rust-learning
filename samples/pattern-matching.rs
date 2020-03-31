// Like C switch
fn main() {
  let number = 11;

  println!("Tell me about {}", number);
  match number {
    1 => println!("One"),
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    13..=19 => println!("A teen"),
    _ => println!("The rest of cases"),
  }
}