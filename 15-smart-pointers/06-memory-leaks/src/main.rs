use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    // nstead of having the ability to modify the i32 value as we did in Listing
    // 15-24, we want to modify which List value a Cons variant is pointing to.
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn node_ds() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // When you call Rc::downgrade, you get a smart pointer of type Weak<T>.
        // Instead of increasing the strong_count in the Rc<T> instance by 1,
        // calling Rc::downgrade increases the weak_count by 1. The Rc<T> type
        // uses weak_count to keep track of how many Weak<T> references exist,
        // similar to strong_count. The difference is the weak_count doesn’t
        // need to be 0 for the Rc<T> instance to be cleaned up.

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    // to do anything with the value that a Weak<T> is pointing to, you must make
    // sure the value still exists. Do this by calling the upgrade method on a
    // Weak<T> instance, which will return an Option<Rc<T>>. You’ll get a result
    // of Some if the Rc<T> value has not been dropped yet and a result of None
    // if the Rc<T> value has been dropped.
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    node_ds()
}
