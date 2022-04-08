// https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html
fn main() {
    mutability();
    println!("\n--------\n");
    shadowing();
    println!("\n--------\n");
    shadowing_diff_type();
}

fn mutability() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowing() {
    // Shadowing is different from marking a variable as mut, because we’ll get
    // a compile-time error if we accidentally try to reassign to this variable
    // without using the let keyword. By using let, we can perform a few
    // transformations on a value but have the variable be immutable after those
    // transformations have been completed.

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

fn shadowing_diff_type() {
    // The other difference between mut and shadowing is that because we’re
    // effectively creating a new variable when we use the let keyword again, we can
    // change the type of the value but reuse the same name.

    let spaces = "abcdefg";
    println!("spaces is {}", spaces);
    let spaces = spaces.len();
    println!("now spaces is {}", spaces);
}
