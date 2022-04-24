use minigrep::Config;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    // Using unwrap_or_else allows us to define some custom, non-panic! error
    // handling
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);

        // A nonzero exit status is a convention to signal to the process that
        // called our program that the program exited with an error state.
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // we prefix the run function with our crate name
    if let Err(e) = minigrep::run(config) {
        // he run function doesn’t return a value that we want to unwrap in the
        // same way that Config::new returns the Config instance.

        // Because run returns () in the success case, we only care about
        // detecting an error, so we don’t need unwrap_or_else to return the
        // unwrapped value because it would only be ().
        println!("Application error: {}", e);

        process::exit(1);
    }
}
