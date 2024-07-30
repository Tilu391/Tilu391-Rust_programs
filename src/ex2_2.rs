use std::io;
use rand::Rng;

fn main() {
    // Create a random number generator and generate a number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("Input your guess:");

        // Create a mutable String to hold the user's guess
        let mut guess = String::new();

        // Read the user's input
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Try to parse the input as a u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        // Increase the attempts counter
        attempts += 1;

        // Check the user's guess
        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else {
            println!("You guessed it in {} attempts!", attempts);
            break;
        }
    }
}
