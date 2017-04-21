use std::io;                               // Using the std::io library provides you with a number of useful io-related features, including the functionality to accept user input

fn main() {                                // The fn syntax declares a new function, the () indicate there are no parameters, and { starts the body of the function
    println!("Guess the number!");         // println! is a macro that prints a string to the screen
    println!("Please input your guess.");
    let mut guess = String::new();         // storing values with variables [org]
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
