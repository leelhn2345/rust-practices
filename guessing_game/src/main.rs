use rand::thread_rng;
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..101);

    println!("The secrent number is {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        // references need to be mutable, hence "&mut guess" and not "& guess"
        .expect("failed to read line");

    println!("You guessed: {}", guess);
}
