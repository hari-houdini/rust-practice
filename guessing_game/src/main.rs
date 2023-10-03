// Although this is a standard library, Rust automatically imports some by default called Prelude
// https://doc.rust-lang.org/std/prelude/index.html
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// A crate is a compilation unit in Rust. Consider it like packages in JS.
// https://doc.rust-lang.org/rust-by-example/crates.html
// This project could be considered a binary crate.
// The rand crate we included in the dependencies could considered a library crate
fn main() {
  println!("Guess the number between 1 and 10");
  let secret_number = rand::thread_rng().gen_range(1..=10);

  // Just plain `loop` would cause an infinite loop
  loop {
    println!("Please input the number: ");

    // Use mut to create mutable variables
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess) // For immutables, use &guess instead of &mut guess
    .expect("Failed to read line");

    // Rust cannot compare a string and a number type
    // By default Rust considers an integer to be `i32` -> signed 32-bit number
    // Here we convert it to `u32` because the rand num is a unsigned 32-bit number

    // Pressing Enter completes read_line(). So string would be like `5\n` on Linux/Mac
    // and `5\r\n` on Windows. trim() removes it.

    // parse() is used to type cast string to another type.
    // Explicitly mentioning `: u32` will parse the string to u32 integer
    // The underscore `_` is a catchall value

    // Also redeclaring in Rust reshadows the existing variable
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Error! Only enter a number");
        continue;
      }
    };

    println!("You guessed: {guess}");

    // match is the equivalent of switch ??
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
