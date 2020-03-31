fn is_odd(n: i32) -> bool {
  n % 2 == 1
}

fn main() {
  println!("Find the sum of all the squared odd numbers under 1000");
  let upper = 1000;

  let mut acc = 0;
  for n in 0.. {
    let n_squared = n * n;

    if n_squared >= upper {
      break;
    } else if is_odd(n_squared) {
      acc += n_squared;
    }
  }

  println!("Imperative style: {}", acc);

  // Functional approach
  let sum_of_squared_odd_numbers: i32 =
    (0..).map(|n| n * n)                          // all natural numbers squared
      .take_while(|&n_squared| n_squared < upper) // below upper limit
      .filter(|&n_squared| is_odd(n_squared))     // that are odd
      .fold(0, |acc, n_squared| acc + n_squared); // sum
  println!("Functional style: {}", sum_of_squared_odd_numbers);
}
