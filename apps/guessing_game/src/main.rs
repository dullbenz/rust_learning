use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game!");

    // Initialize random number generator
    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);
    
    loop {
        // Generate the secret number
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = guess.trim().parse().expect("Guess should be convertible to a number!");
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Sorry, too low! Try again"),
            Ordering::Greater => println!("Sorry, to high! Try again"),
            Ordering::Equal => {
                println!("Perfect, you guessed correctly.");
                break;
            }
        }
    }
}
