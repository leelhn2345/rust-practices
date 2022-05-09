// ! Rc<T>, the Reference Counted Smart Pointer
// In the majority of cases, ownership is clear: you know exactly which variable
// owns a given value. However, there are cases when a single value might have
// multiple owners.
// $ For example, in graph data structures, multiple edges might point to the
// same node, and that node is conceptually owned by all of the
// edges that point to it. A node shouldn’t be cleaned up unless it doesn’t have
// any edges pointing to it.

// ! To enable multiple ownership, Rust has a type called Rc<T>, which is an
// ! abbreviation for reference counting. The Rc<T> type keeps track of the number
// of references to a value to determine whether or not the value is still in
// use. If there are zero references to a value, the value can be cleaned up
// without any references becoming invalid.

// ! We use the Rc<T> type when we want to allocate some data on the heap for
// multiple parts of our program to read and we can’t determine at compile time
// which part will finish using the data last.

// * Note that Rc<T> is only for use in single-threaded scenarios.

// ! error example
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a));
// }
// ! ///////////////////////////

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // clone reference of a
    let c = Cons(4, Rc::clone(&a)); // clone reference of a
                                    // a now has 2 pointers pointing towards it
                                    // `Rc::clone`` doesn't do deep cloning, it
                                    // only increments the reference count
    println!("{:?}", c);

    // * //////////////////////////////

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // ! We can see that the Rc<List> in a has an initial reference count of 1;
    // then each time we call clone, the count goes up by 1. When c goes out of
    // scope, the count goes down by 1.

    // Via immutable references, Rc<T> allows you to share data between multiple
    // parts of your program for reading only. If Rc<T> allowed you to have
    // multiple mutable references too, you might violate one of the borrowing
    // rules - data races and inconsistencies
}
