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

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
