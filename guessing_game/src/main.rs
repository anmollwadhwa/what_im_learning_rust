use std::io;


fn main() {
    println!("Welcome to the guessing game");
    println!("Guess a number");
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read the line");
    println!("You guessed : {guess}");
}