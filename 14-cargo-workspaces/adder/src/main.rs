//! # main adder crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

use add_one;

/// main function
fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
