enum List {
    // ! To figure out how much memory the List type needs, the compiler looks
    // at the variants, starting with the Cons variant. The Cons variant holds a
    // value of type i32 and a value of type List, and this process continues
    // infinitely
    // Cons(i32, List),

    // * Because a Box<T> is a pointer, Rust always knows how much space a
    //   Box<T> needs: a pointer’s size doesn’t change based on the amount of
    //   data it’s pointing to.
    Cons(i32,Box<List>)
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // let b = Box::new(5);
    // println!("b = {}, and it is stored on heap", b);
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    // * Box::new tells compiler to store data on heap
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // The Box<T> type is a smart pointer because it implements the Deref trait,
    // which allows Box<T> values to be treated like references. When a Box<T>
    // value goes out of scope, the heap data that the box is pointing to is
    // cleaned up as well because of the Drop trait implementation. 
}
