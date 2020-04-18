fn give_commoner(gift: Option<&str>) {
  match gift {
    Some("snake") => println!("Yuck!"),
    Some(inner) => println!("{}? How nice.", inner),
    None => println!("Rata!")
  }
}

fn give_princess(gift: Option<&str>) {
  // unwrap returns a panic when its receive None
  let inside = gift.unwrap();
  if inside == "snake" { panic!("Aaaaaaa!" ); }

  println!("I love {}s!", inside);
}

fn main() {
  let food = Some("cabbage");
  let snake = Some("snake");
  let void = None;

  give_commoner(food);
  give_commoner(snake);
  give_commoner(void);

  give_princess(food);
  give_princess(void);
}