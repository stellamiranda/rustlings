// ability to accept user input.
use std::io;

fn main() {
	println!("Guess the number!");

	println!("Please input your guess.");

	// create a variable to store the user input
	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	println!("You guessed: {guess}");
}
