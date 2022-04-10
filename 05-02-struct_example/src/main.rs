#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    meaningful_struct();
    useful_functionality_with_derived_traits();
    dbg();
}

fn meaningful_struct() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("Hello, World!")
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
fn useful_functionality_with_derived_traits() {
    // The println! macro can do many kinds of formatting, and by default, the
    // curly brackets tell println! to use formatting known as Display: output
    // intended for direct end user consumption. The primitive types we’ve seen
    // so far implement Display by default, because there’s only one way you’d
    // want to show a 1 or any other primitive type to a user. But with structs,
    // the way println! should format the output is less clear because there are
    // more display possibilities: Do you want commas or not? Do you want to
    // print the curly brackets? Should all the fields be shown? Due to this
    // ambiguity, Rust doesn’t try to guess what we want, and structs don’t have
    // a provided implementation of Display to use with println! and the {}
    // placeholder.

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Putting the specifier :? inside the curly brackets tells println! we want
    // to use an output format called Debug. The Debug trait enables us to print
    // our struct in a way that is useful for developers so we can see its value
    // while we’re debugging our code.
    println!("rect1 is {:#?}", rect1);

    // Rust does include functionality to print out debugging information, but
    // we have to explicitly opt in to make that functionality available for our
    // struct. To do that, we add the outer attribute #[derive(Debug)] just
    // before the struct definition
}

fn dbg() {
    // Another way to print out a value using the Debug format is to use the
    // dbg! macro, which takes ownership of an expression, prints the file and
    // line number of where that dbg! macro call occurs in your code along with
    // the resulting value of that expression, and returns ownership of the
    // value.

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
