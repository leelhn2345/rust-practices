use std::{error::Error, fs};
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");

            //  Our error values will always be string literals that have the
            //  'static lifetime.
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // call the clone method on the values.
        // This will make a full copy of the
        // data for the Config instance to own, which takes more time and memory
        // than storing a reference to the string data. However, cloning the data
        // also makes our code very straightforward because we don’t have to manage
        // the lifetimes of the references; in this circumstance, giving up a little
        // performance to gain simplicity is a worthwhile trade-off.

        // ! As you become more experienced with Rust, it’ll be easier to start
        // ! with the most efficient solution, but for now, it’s perfectly acceptable
        // ! to call clone.

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> means the function will return a type that implements the
    // Error trait, but we don’t have to specify what particular type the return
    // value will be. This gives us flexibility to return error values that may
    // be of different types in different error cases.
    // ! `dyn` keyword is short for "dynamic"

    // $  ? will return the error value from the current function for the caller to handle.
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
