use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // Okay, first thing is let's initialize a string to store our guess
    println!("Guess the number!"); 
    println!("Please input your guess.");


    let mut guess = String::new();

    // Process inputs and unwrap it
    io::stdin()  // Get the standard input
        .read_line(&mut guess) // Process the input line
        .expect("Failed to read line"); // Print error message if we can't unwrap answer


    // Test our handling function by outputting our result
    println!("You guessed: {}", guess);

    let secret_number = rand::thread_rng().gen_range(1..101);

    // Handle all possible cases for the Result
    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Please input a number.");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
        }
    }

    // TODO Create a looped version 

}
