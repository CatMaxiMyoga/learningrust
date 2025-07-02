use clearscreen::clear;
use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
  let min = 0;
  let max = 1000 - 1;

  clear().expect("Failed to clear screen");
  println!(
    "{}",
    format!("Guess the number! ({min}-{max})")
      .yellow()
      .blink()
      .bold()
      .underline()
  );
  let secret: u32 = rand::rng().random_range(min..=max);
  let mut guesses: u32 = 0;

  loop {
    let mut guess: String = String::new();
    guesses += 1;

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line!");
    print!("\x1B[1A\r");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret) {
      Ordering::Less => println!("{}", format!("{guess} ▲").green().bold()),
      Ordering::Greater => println!("{}", format!("{guess} ▼").red().bold()),
      Ordering::Equal => {
        println!(
          "{}",
          format!("{guess} is correct!").cyan().bold().underline()
        );
        break;
      }
    }
  }

  println!(
    "{}",
    format!("Took {guesses} guesses!").white().dimmed().bold()
  );
  println!("{}", "Enter to restart".white().dimmed().italic());
  io::stdin().read_line(&mut String::new()).unwrap();
  io::stdout().flush().unwrap();
  main();
}
