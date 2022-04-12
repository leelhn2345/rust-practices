// https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
use std::io;
fn main() {
    floating_point_types();
    numeric_operations();
    character_type();
    tuple();
    tuple_index();
    array();
}

fn floating_point_types() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}

fn character_type() {
    //  Note that we specify char literals with single quotes, as opposed to
    //  string literals, which use double quotes.

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

fn tuple() {
    // This program first creates a tuple and binds it to the variable tup. It
    // then uses a pattern with let to take tup and turn it into three separate
    // variables, x, y, and z. This is called destructuring.
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}

fn tuple_index() {
    // We can also access a tuple element directly by using a period (.)
    // followed by the index of the value we want to access.

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

fn array() {
    // arrays in Rust have a fixed length

    // An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.
    let a = [1, 2, 3, 4, 5];

    // arrays are more useful when you know the number of elements will not need
    // to change. For example, if you were using the names of the month in a
    // program, you would probably use an array rather than a vector because you
    // know it will always contain 12 elements
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // the array named `a` will contain 5 elements that will all be set to the
    // value 3 initially
    let a = [3; 5];
}

fn invalid_array_element_access() {
    // accessing array index 10 will lead to a runtime error.
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
