// Most errors aren’t serious enough to require the program to stop entirely.
// Sometimes, when a function fails, it’s for a reason that you can easily
// interpret and respond to. For example, if you try to open a file and that
// operation fails because the file doesn’t exist, you might want to create the
// file instead of terminating the process.

//         enum Result<T, E> {
//              Ok(T),
//              Err(E),
//         }

// T represents the type of the value that will be returned in a success case
// within the Ok variant, and E represents the type of the error that will be
// returned in a failure case within the Err variant.

//  The generic parameter T has been filled in here with the type of the success
//  value, std::fs::File, which is a file handle. The type of E used in the
//  error value is std::io::Error.

use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // error();
    // matching_on_different_errors();

    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}

fn error() {
    let f = File::open("hello.txt");

    // In the case where File::open succeeds, the value in the variable f will
    // be an instance of Ok that contains a file handle. In the case where it
    // fails, the value in f will be an instance of Err that contains more
    // information about the kind of error that happened.

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn matching_on_different_errors() {
    let f = File::open("hello.txt");

    // The type of the value that File::open returns inside the Err variant is
    // io::Error
    // This struct has a method kind that we can call to get an io::ErrorKind
    // value. The enum io::ErrorKind is provided by the standard library and has
    // variants representing the different kinds of errors that might result
    // from an io operation.

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                // File::create could also fail, we need a second arm in the
                // inner match expression
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    // That’s a lot of match! The match expression is very useful but also very
    // much a primitive. In Chapter 13, you’ll learn about closures, which are
    // used with many of the methods defined on Result<T, E>. These methods can
    // be more concise than using match when handling Result<T, E> values in
    // your code.

    // fn main() {
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
    // }
}

fn shortcuts_for_panic_on_error() {
    // The unwrap method is a shortcut method implemented just like the match
    // expression

    // If the Result value is the Ok variant, unwrap will return the value
    // inside the Ok. If the Result is the Err variant, unwrap will call the
    // panic! macro for us.

    // let f = File::open("hello.txt").unwrap();

    // Similarly, the expect method lets us also choose the panic! error
    // message. Using expect instead of unwrap and providing good error messages
    // can convey your intent and make tracking down the source of a panic
    // easier.

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn propagating_errors() -> Result<String, io::Error> {
    // When a function’s implementation calls something that might fail, instead
    // of handling the error within the function itself, you can return the
    // error to the calling code so that it can decide what to do. This is known
    // as propagating the error and gives more control to the calling code,
    // where there might be more information or logic that dictates how the
    // error should be handled than what you have available in the context of
    // your code.
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

    // The function is returning a value of the type Result<T, E>
    // where the generic parameter T has been filled in with the concrete type
    // String, and the generic type E has been filled in with the concrete type
    // io::Error. If this function succeeds without any problems, the code that
    // calls this function will receive an Ok value that holds a String—the
    // username that this function read from the file. If this function
    // encounters any problems, the calling code will receive an Err value that
    // holds an instance of io::Error that contains more information about what
    // the problems were.
}

fn shortcut_for_propagating_errors() -> Result<String, io::Error> {
    // This pattern of propagating errors is so common in Rust that Rust
    // provides the question mark operator ? to make this easier.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);

    // OR

    File::open("hello.txt")?.read_to_string(&mut s)?;
    return Ok(s);

    // The ? placed after a Result value is defined to work in almost the same
    // way as the match expressions we defined to handle the Result values

    // If the value of the Result is an Ok, the value inside the Ok will get
    // returned from this expression, and the program will continue. If the
    // value is an Err, the Err will be returned from the whole function as if
    // we had used the return keyword so the error value gets propagated to the
    // calling code.

    // Reading a file into a string is a fairly common operation, so the
    // standard library provides the convenient fs::read_to_string function that
    // opens the file, creates a new String, reads the contents of the file,
    // puts the contents into that String, and returns it.

    // using fs::read_to_string doesn’t give us the opportunity to explain all
    // the error handling, so we did it the longer way first.

    fs::read_to_string("hello.txt");

    // The ? operator can only be used in functions whose return type is
    // compatible with the value the ? is used on, Result type
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    // The error message also mentioned that ? can be used with Option<T> values
    // as well. As with using ? on Result, you can only use ? on Option in a
    // function that returns an Option. The behavior of the ? operator when
    // called on an Option<T> is similar to its behavior when called on a
    // Result<T, E>: if the value is None, the None will be returned early from
    // the function at that point. If the value is Some, the value inside the
    // Some is the resulting value of the expression and the function continues.

    text.lines().next()?.chars().last()

    // This function returns Option<char> because it’s possible that there is a
    // character there, but it’s also possible that there isn’t.

    // it’s possible that the first line is the empty string
}

fn main_2() {
    // let f = File::open("hello.txt")?;

    // When we compile this code, we get the following error message
    // This error points out that we’re only allowed to use the ? operator in a
    // function that returns Result, Option, or another type that implements
    // FromResidual.

    fn wow() -> Result<(), Box<dyn Error>> {
        // The Box<dyn Error> type is a trait object, which means "any kind of error".
        let f = File::open("hello.txt")?;

        Ok(())
    }
}
