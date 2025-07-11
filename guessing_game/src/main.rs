use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please guess the output");
    let mut guess= String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to real line");

    println!("You guessed: {guess}");
}

// last read - https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#printing-values-with-println-placeholders