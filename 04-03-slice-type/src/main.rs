fn main() {
    // Slices let you reference a contiguous sequence of elements in a
    // collection rather than the whole collection. A slice is a kind of
    // reference, so it does not have ownership.

    println!("Hello, world!");

    first_word();
    string_slices();
    proper_first_word();
}

// We’re returning a usize on its own, but it’s only a meaningful number in the
// context of the &String. In other words, because it’s a separate value from
// the String, there’s no guarantee that it will still be valid in the future.
fn first_word(s: &String) -> usize {
    // Because we need to go through the String element by element and check
    // whether a value is a space, we’ll convert our String to an array of bytes
    // using the as_bytes method

    let bytes = s.as_bytes();

    // we specify a pattern that has i for the index in the tuple and &item for
    // the single byte in the tuple. Because we get a reference to the element
    // from .iter().enumerate(), we use & in the pattern
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
    // This program compiles without any errors and would also do so if we used
    // word after calling s.clear(). Because word isn’t connected to the state
    // of s at all, word still contains the value 5. We could use that value 5
    // with the variable s to try to extract the first word out, but this would
    // be a bug because the contents of s have changed since we saved 5 in word.
}

fn string_slices() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // With Rust’s .. range syntax, if you want to start at index zero, you can
    // drop the value before the two periods. In other words, these are equal:
    let slice = &s[0..2];
    let slice = &s[..2];

    // By the same token, if your slice includes the last byte of the String,
    // you can drop the trailing number. That means these are equal:
    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    // You can also drop both values to take a slice of the entire string. So
    // these are equal:
    let slice = &s[0..len];
    let slice = &s[..];
}

// With all this information in mind, let’s rewrite first_word to return a
// slice. The type that signifies “string slice” is written as &str
// Now when we call first_word, we get back a single value that is tied to the
// underlying data. The value is made up of a reference to the starting point of
// the slice and the number of elements in the slice.
fn proper_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let zzz = "Dsdq";
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
