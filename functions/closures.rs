fn main() {
  fn function (i: i32) -> i32 {
    i + 1
  }

  let closure_annotated = |i: i32| -> i32 { i + 1 };
  let closure_inferred = |i | i + 1;

  let i = 1;
  println!("function: {}", function(i));
  println!("closure_annotated: {}", closure_annotated(i));
  println!("closure_inferred: {}", closure_inferred(i));

  let one = || 1;
  println!("closure returning one: {}", one());

  fn apply<F>(f: F) where
    F: FnOnce() {
      f();
    }

  let diary = || {
    println!("Hello Multiverse!");
  };

  apply(diary);

  fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
      f(3)
    }

  let double = |x| 2 * x;
  println!("3 doubled: {}", apply_to_3(double));
}