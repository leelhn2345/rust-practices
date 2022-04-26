// closures can capture environment and access variables from scope in which
// they're defined

// Here, even though x is not one of the parameters of equal_to_x, the
// equal_to_x closure is allowed to use the x variable that’s defined in the
// same scope that equal_to_x is defined in.

fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

// ! code won't compile for this
// When a closure captures a value from its environment, it uses memory to store
// the values for use in the closure body. This use of memory is overhead that
// we don’t want to pay in more common cases where we want to execute code that
// doesn’t capture its environment. Because functions are never allowed to
// capture their environment, defining and using functions will never incur this
// overhead.
fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool {
        z == x
    }

    let y = 4;

    assert!(equal_to_x(y));
}
