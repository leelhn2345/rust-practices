fn main() {
    println!("Hello, world!");
    generic_point();
}

// We read this definition as: the function largest is generic over some type T.
// This function has one parameter named list, which is a slice of values of
// type T. The largest function will return a value of the same type T.
fn largest<T>(list: &[T]) -> T {
    // To parameterize the types in the new function we’ll define, we need to
    // name the type parameter, just as we do for the value parameters to a
    // function. You can use any identifier as a type parameter name. But we’ll
    // use T because, by convention, parameter names in Rust are short, often
    // just a letter

    // hen we use a parameter in the body of the function, we have to declare
    // the parameter name in the signature so the compiler knows what that name
    // means. Similarly, when we use a type parameter name in a function
    // signature, we have to declare the type parameter name before we use it.
    // To define the generic largest function, place type name declarations
    // inside angle brackets, <>, between the name of the function and the
    // parameter list, like this

    list[0]
}

// We can also define structs to use a generic type parameter in one or more
// fields using the <> syntax.
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn generic_point() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!(
        "{:#?}, {:#?}, {:#?}",
        both_integer, both_float, integer_and_float
    );
}

fn method_definitions() {
    struct Point<T> {
        x: T,
        y: T,
    }

    // Note that we have to declare T just after impl so we can use it to
    // specify that we’re implementing methods on the type Point<T>.
    // * By declaring T as a generic type after impl, Rust can identify that the
    // * type in the angle brackets in Point is a generic type rather than a
    // * concrete type.
    // Because this is declaring the generic again, we could have
    // chosen a different name for the generic parameter than the generic
    // parameter declared in the struct definition, but using the same name is
    // conventional.

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    // This code means the type Point<f32> will have a method named
    // distance_from_origin and other instances of Point<T> where T is not of
    // type f32 will not have this method defined. The method measures how far
    // our point is from the point at coordinates (0.0, 0.0) and uses
    // mathematical operations that are available only for floating point types.
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

fn mixup() {
    // Generic type parameters in a struct definition aren’t always the same as
    // those you use in that struct’s method signatures. Example below uses the
    // generic types X1 and Y1 for the Point struct and X2 Y2 for the mixup
    // method signature to make the example clearer. The method creates a new
    // Point instance with the x value from the self Point (of type X1) and the
    // y value from the passed-in Point (of type Y2).

    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
