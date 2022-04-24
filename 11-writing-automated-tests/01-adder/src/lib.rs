#[cfg(test)]
mod tests {
    // Note the #[test] annotation before the fn line: this attribute indicates
    // this is a test function, so the test runner knows to treat this function
    // as a test

    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_ne() {
        let result = 2 + 2;
        assert_ne!(result, 5);
    }

    #[test]
    fn make_this_test_fail() {
        panic!("Make this test fail");
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    //  We give the assert! macro an argument that evaluates to a Boolean. If
    //  the value is true, assert! does nothing and the test passes. If the
    //  value is false, the assert! macro calls the panic! macro, which causes
    //  the test to fail.
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // ! test function with custom error message
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // ! expected panic! with custom panic message
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
        // Writing tests so they return a Result<T, E> enables you to use the
        // question mark operator in the body of tests, which can be a
        // convenient way to write tests that should fail if any operation
        // within them returns an Err variant.

        // You can’t use the #[should_panic] annotation on tests that use
        // Result<T, E>. To assert that an operation returns an Err variant,
        // don’t use the question mark operator on the Result<T, E> value.
        // Instead, use assert!(value.is_err()).

        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

pub fn greeting(name: &str) -> String {
    // String::from("Hello!")
    format!("Hello {}!", name)
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug, PartialEq)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}
