# Learning Rust

---

> [!Note]
> I will be making multiple small projects to learn how to 
> use Rust and how it works. Each commit to the main branch will be a new
> project. Each project will be written in a different branch until merged into
> the main branch before starting the next project.

---

# Guessing Game + Binary Guesser Helper

## Guessing Game
- Generates a random number between 0 and 1000 (exclusive)
- Prompts the user to guess the number
- Provides feedback on whether the number is higher or lower than the
guess
- Continues until the user guesses the number
- Displays the number of attempts it took to guess the number
- Fully colorized output

## Binary Guesser Helper
- Asks for the minimum and maximum number
- Displays the maximum amount of guesses it can take to find the number
- Gives the user guesses based on binary search (halving the range each guess)
- Asks for feedback on whether the number searched for is higher or lower
- Continues until either restarted manually by inputting `3` or by reaching 
  the maximum amount of guesses for that range, in which case the number has
  definitely been found.
- Somewhat colorized output

## How to run
### Guessing Game
```bash
cargo run --release --bin testproj
```
Exit using CTRL+C

### Binary Guesser Helper
```bash
cargo run --release --bin binguesshelp
```
Exit using CTRL+C