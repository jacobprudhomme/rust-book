use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..101);  // Or 1..=100

    // println!("The secret number is: {}", secret_number);

    loop {  // Infinite loop
        println!("Please input your guess:");

        let mut guess = String::new();  // Mutable variable (immutable by default)

        io::stdin()
            .read_line(&mut guess)  // Pass mutable reference to guess (immutable by default)
            .expect("Failed to read line");  // Method on io::Result type, an enum with variants Ok and Err, that displays passed message if Err, and returns value within Ok otherwise

        // You can shadow the previous value of guess! Useful for changing types
        let guess: u32 = match guess.trim().parse() {  // Type annotation (u32, unsigned 32-bit int). parse() needs this to determine what to parse string as
            Ok(num) => num,
            Err(_) => {
                println!("Please type in numbers only.");
                continue;  // Restart at beginning of loop
            },
        };

        println!("You guessed: {}", guess);
        // Can use multiple placeholders like so:
        // println!("x = {} and y = {}", val1, val2);

        // Ordering is also an enum, with variants Less, Equal and Greater
        match guess.cmp(&secret_number) {  // Pattern-matching
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  // Break from loop
            },
        }
    }
}
