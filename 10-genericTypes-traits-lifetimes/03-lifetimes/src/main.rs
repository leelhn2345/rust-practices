fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// The function signature now tells Rust that for some lifetime 'a, the function
// takes two parameters, both of which are string slices that live at least as
// long as lifetime 'a. The function signature also tells Rust that the string
// slice returned from the function will live at least as long as lifetime 'a.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // the generic lifetime 'a will get the concrete lifetime that is equal to
    // the smaller of the lifetimes of x and y.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn lifetime_annotation_syntax() {
//     &i32        // a reference
//     &'a i32     // a reference with an explicit lifetime
//     &'a mut i32 // a mutable reference with an explicit lifetime
// }

// ! generic type parametes, trait bounds, and lifetimes together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
