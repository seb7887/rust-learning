use deeply::nested::hello as hello;

mod deeply {
  pub mod nested {
    pub fn hello() {
      println!("Hello World!");
    }
  }
}

fn main() {
  hello();

  // println!("Entering block");
  // {
  //   use crate::deeply::nested::hello;
  //   hello();

  //   println!("Leaving block");
  // }
}
