use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static LOREM: &str = "Do what thou wilt shall be the whole of the Law";

fn main() {
  let path = Path::new("out/lorem.txt");
  let display = path.display();

  let mut file = match File::create(&path) {
    Err(why) => panic!("couldn't write file {}", why.description()),
    Ok(file) => file,
  };

  match file.write_all(LOREM.as_bytes()) {
    Err(_why) => panic!("error"),
    Ok(_) => println!("success! wrote to {}", display),
  }
}