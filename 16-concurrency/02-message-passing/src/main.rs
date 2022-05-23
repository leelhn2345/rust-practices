use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// ! mpsc stands for multiple producer, single consumer
// In short, the way Rust’s standard library implements channels means a channel
// can have multiple sending ends that produce values but only one receiving end
// that consumes those values.

fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
fn multiple_value_tx() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn tx_rx() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // calling unwrap to panic in case of an error
        // refer to chapter 9 for real application error handling

        // ! this causes error
        // * println!("val is {}", val);
        // Allowing this would be a bad idea: once the value has been sent to
        // another thread, that thread could modify or drop it before we try to
        // use the value again. Potentially, the other thread’s modifications
        // could cause errors or unexpected results due to inconsistent or
        // nonexistent data.

        // * ts.send(value) function takes ownership of parameters
        // this stops us from accidentally using the value again after sending it
    });

    // The receiving end of a channel has two useful methods: recv and try_recv
    // * recv will block the main thread's execution and wait until a value is
    // sent down the channel
    // $ The try_recv method doesn’t block, but will instead return a Result<T, E>

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
