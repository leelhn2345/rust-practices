// https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            // references are immutable by default
            // &mut means mutable references
            // read_line puts whatever the user inputs into the string, and also
            // returns a value - io::Result
            .read_line(&mut guess)
            // handling potential failure with result type
            // if there's no expect, Rust warns that you haven’t used the Result value returned from
            // read_line, indicating that the program hasn’t handled a possible
            // error.
            .expect("Failed to read line");
        // If parse is able to successfully turn the string into a number, it
        // will return an Ok value that contains the resulting number. That Ok
        // value will match the first arm’s pattern, and the match expression
        // will just return the num value that parse produced and put inside the
        // Ok value. That number will end up right where we want it in the new
        // guess variable we’re creating.

        // The underscore, _, is a catchall value; in this example, we’re saying
        // we want to match all Err values, no matter what information they have
        // inside them. So the program will execute the second arm’s code,
        // continue, which tells the program to go to the next iteration of the
        // loop and ask for another guess.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // This line prints the string that now contains the user’s input. The {}
        // set of curly brackets is a placeholder
        println!("You guessed: {}", guess);

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
