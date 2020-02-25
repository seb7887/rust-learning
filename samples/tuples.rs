use std::fmt::{self, Formatter, Display };

fn reverse(pair: (i32, bool)) -> (bool, i32) {
  let (integer, boolean) = pair;

  (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
  }
}

fn transpose(mtx: Matrix) -> Matrix {
  let transposed = Matrix(mtx.0, mtx.2, mtx.1, mtx.3);

  transposed
}

fn main() {
  let long_tuple = (1u8, 2u16, 3u32, 4u64, 'a', true);

  println!("long tuple first value: {}", long_tuple.0);

  let pair = (1, true);
  println!("pair is {:?}", pair);

  println!("the reversed pair is {:?}", reverse(pair));

  // To create one element tuples, the comma is required to tell them apart
  // from a literal surrounded by parentheses
  println!("one element tuple: {:?}", (5u32,));
  println!("just an integer: {:?}", (5u32));

  //tuples can be destructured to create bindings
  let tuple = (1, "hello", 4.5, true);

  let (a, b, c, d) = tuple;
  println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{}", matrix);

  let transposed_matrix = transpose(matrix);
  println!("{}", transposed_matrix);
}