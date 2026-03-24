use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::rng().random_range(1..=100);
  println!("The secret number is: {secret_number}");

  loop {
    println!("Please input your guess.");
    let mut guess = String::new();
    let output = io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");

    // let guess: u32 = guess.trim().parse().expect("Please type a number!");
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(error) => {
        println!("Error: {error}");
        continue;
      }
    };

    println!("byte of input: {output}");
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Equal => {
        println!("You win!");
        break
      },
      Ordering::Greater => println!("Too big!"),
    }
  }
}
