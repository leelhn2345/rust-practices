use std::mem::drop;
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // Variables are dropped in the reverse order of their creation, so d was
    // dropped before c.

    dropping_value_early();
}

fn dropping_value_early() {
    // Occasionally, however, you might want to clean up a value early. One
    // example is when using smart pointers that manage locks: you might want to
    // force the drop method that releases the lock so that other code in the
    // same scope can acquire the lock.

    // Rust doesn’t let you call the Drop trait’s drop method manually;
    // ! instead you have to call the std::mem::drop function provided by the
    // ! standard library if you want to force a value to be dropped before the
    // ! end of its scope.

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("      CustomSmartPointer created in sub func.");
    drop(c);
    println!("      CustomSmartPointer dropped before the end of sub func.");
}
