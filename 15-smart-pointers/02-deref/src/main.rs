use std::ops::Deref;

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y.deref());

    // because we implemented the Deref trait on MyBox<T> in Listing 15-10, Rust
    // can turn &MyBox<String> into &String by calling deref.
    // Rust calls deref again to turn the &String into &str, which matches the
    // hello function’s definition.

    // * When the Deref trait is defined for the types involved, Rust will
    // * analyze the types and use Deref::deref as many times as necessary to
    // * get a reference to match the parameter’s type. The number of times that
    // * Deref::deref needs to be inserted is resolved at compile time, so there
    // * is no runtime penalty for taking advantage of deref coercion!

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // ! without deref coercion
    // let m = MyBox::new(String::from("Rust"));
    // hello(&(*m)[..]);
    // The (*m) dereferences the MyBox<String> into a String. Then the & and
    // [..] take a string slice of the String that is equal to the whole string
    // to match the signature of hello. The code without deref coercions is
    // harder to read, write, and understand with all of these symbols involved.
}

// ! How Deref Coercion interacts with Mutability
// Rust does deref coercion when it finds types and trait implementations in three cases:

// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>

// The first two cases are the same except for mutability. The first case states
// that if you have a &T, and T implements Deref to some type U, you can get a
// &U transparently. The second case states that the same deref coercion happens
// for mutable references.

// The third case is trickier: Rust will also coerce a mutable reference to an
// immutable one. But the reverse is not possible: immutable references will
// never coerce to mutable references.

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // We fill in the body of the deref method with &self.0 so deref returns
        // a reference to the value we want to access with the * operator.
        // Recall from the “Using Tuple Structs without Named Fields to Create
        // Different Types” section of Chapter 5 that .0 accesses the first
        // value in a tuple struct. The main function in Listing 15-9 that calls
        // on the MyBox<T> value now compiles, and the assertions pass!
        &self.0
    }
}

// We define a struct named MyBox and declare a generic parameter T, because we
// want our type to hold values of any type. The MyBox type is a tuple struct
// with one element of type T. The MyBox::new function takes one parameter of
// type T and returns a MyBox instance that holds the value passed in.
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// ! implicit deref coercions with functions and methods
// deref coercion is a convenience that rust performs on arguments to functions
// and methods. deref coercion works onyl on types that implement the Deref
// trait. Deref coercion converts a reference to such a type into a referecne to
// another type. . For example, deref coercion can convert &String to &str
// because String implements the Deref trait such that it returns &str.

// Deref coercion happens automatically when we pass a reference to a particular
// type’s value as an argument to a function or method that doesn’t match the
// parameter type in the function or method definition. A sequence of calls to
// the deref method converts the type we provided into the type the parameter
// needs.

// Deref coercion was added to Rust so that programmers writing function and
// method calls don’t need to add as many explicit references and dereferences
// with & and *. The deref coercion feature also lets us write more code that
// can work for either references or smart pointers.

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
