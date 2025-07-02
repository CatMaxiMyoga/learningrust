use clearscreen::clear;
use colored::Colorize;
use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn line_up() {
  print!("\x1B[1A\r");
  io::stdout().flush().unwrap();
}

fn empty_line() {
  print!("\r");
  print!("{}", " ".repeat(200));
  println!();
  io::stdout().flush().unwrap();
}

fn override_line(msg: &str) {
  line_up();
  empty_line();
  print!("{msg}");
  io::stdout().flush().unwrap();
}

fn input(msg: &str, var: &mut i32) {
  let mut string: String = String::new();

  print!("{msg}");
  io::stdout().flush().unwrap();

  *var = loop {
    string.clear();
    io::stdin().read_line(&mut string).unwrap();
    string = string.trim().to_string();

    if string.parse::<i32>().is_ok() {
      break string.parse::<i32>().unwrap();
    }

    override_line(&"Invalid Number!".bold().red());
    sleep(Duration::from_secs(2));
    override_line(msg);
  };
}

fn main() {
  let mut start_min: i32 = 0;
  let mut start_max: i32 = 0;
  let mut min: i32;
  let mut max: i32;
  let mut current: i32;
  let mut mode: i32 = 0;

  input("min >> ", &mut start_min);
  input("max >> ", &mut start_max);

  let max_guesses: u32 =
    ((start_max - start_min + 1) as f64).log2().ceil() as u32;

  loop {
    clear().expect("Failed to clear screen");
    io::stdout().flush().unwrap();
    println!("{start_min} - {start_max}");
    min = start_min;
    max = start_max;
    println!(
      "Type {} or {} accordingly.",
      "1 for down".red().bold(),
      "2 for up".green().bold()
    );
    println!(
      "Type {} with the same max and min.",
      "3 to restart".yellow().bold()
    );
    println!("Will take at most {max_guesses} guesses.\n");
    io::stdout().flush().unwrap();

    loop {
      if min == max {
        current = min;
        break;
      }
      current = max - (max - min) / 2;

      input(
        &format!("{current} ({min}-{max})  >> ").yellow().bold(),
        &mut mode,
      );

      if mode == 1 {
        max = current - 1;
        if current == max {
          current -= 1;
          break;
        }
      } else if mode == 2 {
        min = current + 1;
        if current == min {
          current += 1;
          break;
        }
      } else if mode == 3 {
        break;
      } else {
        line_up();
        empty_line();
        continue;
      }
    }
    if mode == 3 {
      continue;
    }
    println!("{}", format!("The number is {current}").green().bold());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
  }
}
