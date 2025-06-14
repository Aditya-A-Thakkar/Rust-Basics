// Importing IO from the standard library
use std::io::{ self, Write };
// Import rand crate for generation of random numbers
use rand::Rng;

fn main() {
    // Generating a Random number
    let random_number = rand::rng().random_range(1..101);
    println!("Secret number: {}", random_number);

    // An infinite loop which runs until break is used
    'my_loop: loop {
        // Taking Input from user

        /* We define variables as follows:-
        let name :type[Optional] = value;     // immutable
        let mut name :type[Optional] = value; // mutable
         */
        let mut input :String = String::new();

        // println! outputs a new line at the end, but print! does not
        print!("Enter something: ");

        // Flushing the console for correct order of output[optional]
        io::stdout().flush().unwrap();

        // Reading the input given by user and storing it in 'input' variable via a mutable reference
        io::stdin().read_line(&mut input).expect("Failed to read line");

        println!("You entered: {}", input); // Notice the formatting

        // Converting the received input to i32
        // You can use an if statement in a let statement
        let user_number = if input.trim().parse::<i32>().is_ok() {
            input.trim().parse::<i32>().unwrap()
        } else {
            println!("That's not a number!");
            continue 'my_loop;
            // In Rust you can specify which loop to continue/break, this helps when handling nested loops
        };
        println!("Your number: {}", user_number);

        // Comparing the user_number with random_number
        if user_number == random_number {
            print!("Yes, you guessed the number.");
            break 'my_loop;
        } else if user_number > random_number {
            println!("No, your number is greater than the secret number.");
        } else {
            println!("No, your number is less than the secret number.");
        }
    }
}