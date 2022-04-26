use minigrep::Config;
use std::{env, process};
fn main() {
    // Using unwrap_or_else allows us to define some custom, non-panic! error
    // handling
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // ! The standard library provides the eprintln! macro that prints to
        // ! the standard error stream
        eprintln!("Problem parsing arguments: {}", err);

        // A nonzero exit status is a convention to signal to the process that
        // called our program that the program exited with an error state.
        process::exit(1);
    });

    // we prefix the run function with our crate name
    if let Err(e) = minigrep::run(config) {
        // the run function doesn’t return a value that we want to unwrap in the
        // same way that Config::new returns the Config instance.

        // Because run returns () in the success case, we only care about
        // detecting an error, so we don’t need unwrap_or_else to return the
        // unwrapped value because it would only be ().
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
