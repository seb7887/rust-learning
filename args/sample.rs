use std::env;

fn increase(number: i32) {
  println!("{}", number + 1);
}

fn decrease(number: i32) {
  println!("{}", number - 1);
}

fn help() {
  println!("usage: sample <string>. sample {{increase|decrease}} <integer>");
}

fn main() {
  let args: Vec<String> = env::args().collect();

  match args.len() {
    1 => {
      println!("My name is 'sample'. Try passing some arguments!");
    },
    2 => {
      match args[1].parse() {
        Ok(42) => println!("This is the answer!"),
        _ => println!("Incorrect!"),
      }
    },
    3 => {
      let cmd = &args[1];
      let num = &args[2];

      let number: i32 = match num.parse() {
        Ok(n) => {
          n
        },
        Err(_) => {
          eprintln!("error: second argument missing!");
          help();
          return;
        },
      };

      match &cmd[..] {
        "increase" => increase(number),
        "decrease" => decrease(number),
        _ => {
          eprintln!("error: invalid command!");
          help();
        },
      }
    },
    _ => {
      help();
    }
  }
}