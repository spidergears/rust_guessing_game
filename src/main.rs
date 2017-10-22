extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess a number!");
  let secret_number = rand::thread_rng().gen_range(1, 100);

  println!("Please input your guess");


  loop {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You have guessed {}", guess);

    match guess.cmp(&secret_number){
      Ordering::Less => {
        println!("Too low!");
        continue;
      },
      Ordering::Greater => {
        println!("Too high!");
        continue;
      },
      Ordering::Equal => {
        println!("Congrarulations!! You won.");
        break;
      },
    }
  }
}
