// So how do you decide when you should call panic! and when you should return
// Result? When code panics, there’s no way to recover. You could call panic!
// for any error situation, whether there’s a possible way to recover or not,
// but then you’re making the decision that a situation is unrecoverable on
// behalf of the calling code. When you choose to return a Result value, you
// give the calling code options. The calling code could choose to attempt to
// recover in a way that’s appropriate for its situation, or it could decide
// that an Err value in this case is unrecoverable, so it can call panic! and
// turn your recoverable error into an unrecoverable one. Therefore, returning
// Result is a good default choice when you’re defining a function that might
// fail.

// If a method call fails in a test, you’d want the whole test to fail, even if
// that method isn’t the functionality under test. Because panic! is how a test
// is marked as a failure, calling unwrap or expect is exactly what should
// happen.

use std::net::IpAddr;

fn main() {
    println!("Hello, world!");

    ip_address();
}

fn ip_address() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    // We’re creating an IpAddr instance by parsing a hardcoded string. We can
    // see that 127.0.0.1 is a valid IP address, so it’s acceptable to use
    // unwrap here. However, having a hardcoded, valid string doesn’t change the
    // return type of the parse method: we still get a Result value, and the
    // compiler will still make us handle the Result as if the Err variant is a
    // possibility because the compiler isn’t smart enough to see that this
    // string is always a valid IP address. If the IP address string came from a
    // user rather than being hardcoded into the program and therefore did have
    // a possibility of failure, we’d definitely want to handle the Result in a
    // more robust way instead.
}

fn creating_guess_struct() {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
    // we implement a method named value that borrows self, doesn’t have any
    // other parameters, and returns an i32. This kind of method is sometimes
    // called a getter, because its purpose is to get some data from its fields
    // and return it. This public method is necessary because the value field of
    // the Guess struct is private. It’s important that the value field be
    // private so code using the Guess struct is not allowed to set value
    // directly: code outside the module must use the Guess::new function to
    // create an instance of Guess, thereby ensuring there’s no way for a Guess
    // to have a value that hasn’t been checked by the conditions in the
    // Guess::new function.
}
