use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // ! Multiple ownership with multiple threads
        // Unfortunately, Rc<T> is not safe to share across threads.
        // it doesn’t use any concurrency primitives to make sure that changes
        // to the count can’t be interrupted by another thread.

        // * Fortunately, Arc<T> is a type like Rc<T> that is safe to use in
        //   concurrent situations. The a stands for atomic, meaning it’s an
        //   atomically reference counted type. Atomics are an additional kind
        //   of concurrency primitive

        // You might then wonder why all primitive types aren’t atomic and why
        // standard library types aren’t implemented to use Arc<T> by default.
        // The reason is that thread safety comes with a performance penalty
        // that you only want to pay when you really need to. If you’re just
        // performing operations on values within a single thread, your code can
        // run faster if it doesn’t have to enforce the guarantees atomics
        // provide.

        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn mutex() {
    let m = Mutex::new(5);
    println!("m = {:?}", m);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
