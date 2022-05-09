// Interior mutability is a design pattern in Rust that allows you to mutate
// data even when there are immutable references to that data

// the pattern uses unsafe code inside a data structure to bend Rust’s usual
// rules that govern mutation and borrowing.

// The unsafe code involved is then wrapped in a safe API, and the outer type is
// still immutable.

// ! Difference between Box<T> and RefCell<T>
// Box<T> enforces borrwing rules at compile time
// RefCell<T> enforces it during runtime. Program will panic and exit if rules
// are broken

// ! Recap of Box<T>, Rc<T>, RefCell<T>:
// 1. Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have
//    single owners.
// 2. Box<T> allows immutable or mutable borrows checked at compile time; Rc<T>
//    allows only immutable borrows checked at compile time; RefCell<T> allows
//    immutable or mutable borrows checked at runtime.
// 3. Because RefCell<T> allows mutable borrows checked at runtime, you can
//    mutate the value inside the RefCell<T> even when the RefCell<T> is
//    immutable.

// * there are situations in which it would be useful for a value to mutate
// * itself in its methods but appear immutable to other code.

fn main() {
    println!("Hello, world!");
    multiple_owners();
}

// ! Having multiple owners of mutable data by combining Rc<T> and RefCell<T>
// a common way to use RefCell<T> is in combination with Rc<T>. The latter lets
// us have multiple owners of some data, but it only gives immutable access to
// that data.
// * If we have an Rc<T> that holds a RefCell<T>, we can get a value that have
//   multiple owners and that we can mutate.

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn multiple_owners() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
