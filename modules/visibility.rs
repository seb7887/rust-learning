mod my_mod {
  fn private_fn() {
    println!("This is a private function!");
  }

  pub fn public_fn() {
    println!("This is a public function!");
  }

  // Items can access other items in the same module
  pub fn indirect_access() {
    private_fn();
  }

  // Nested module
  pub mod nested {
    pub fn hello() {
      println!("Hello World!");
    }
  }
}

fn main() {
  my_mod::public_fn();
  my_mod::indirect_access();
  my_mod::nested::hello();
}
