// 2- Formatted print
fn main() {
  println!("{}'s", 93);
  println!("{0} is the {1}, {0} under Will", "Love", "Law");
  println!("{thelema}", thelema="Thelema");
  println!("{} is {:b}", 2, 2);
  println!("{text:>width$}", text="margin", width=6);
  println!("Pi is roughly {pi:.prec$}", prec=3, pi=3.141592);
}