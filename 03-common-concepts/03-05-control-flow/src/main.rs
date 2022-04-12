fn main() {
    println!("Hello, world!");
    non_boolean_types_do_not_get_converted_to_boolean();
    ternary_operator();
    repeating_code_with_loop();
    returning_values_from_loops();
    conditional_while_loops();
    for_loop();
}

fn non_boolean_types_do_not_get_converted_to_boolean() {
    // Unlike languages such as Ruby and JavaScript, Rust will not automatically
    // try to convert non-Boolean types to a Boolean. You must be explicit and
    // always provide if with a Boolean as its condition.
    let number = 3;

    if number {
        println!("number was three");
    }
}

fn ternary_operator() {
    // variables must have a single type, and Rust needs to know at compile time
    // what type the number variable is, definitively. Knowing the type of
    // number lets the compiler verify the type is valid everywhere we use
    // number. Rust wouldn’t be able to do that if the type of number was only
    // determined at runtime

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

fn repeating_code_with_loop() {
    loop {
        println!("again!");
    }
}

fn break_out_of_loop_with_label() {
    // The outer loop has the label 'counting_up, and it will count up from 0 to
    // 2. The inner loop without a label counts down from 10 to 9. The first
    // break that doesn’t specify a label will exit the inner loop only. The
    // break 'counting_up; statement will exit the outer loop.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn returning_values_from_loops() {
    // One of the uses of a loop is to retry an operation you know might fail,
    // such as checking whether a thread has completed its job. You might also
    // need to pass the result of that operation out of the loop to the rest of
    // your code. To do this, you can add the value you want returned after the
    // break expression you use to stop the loop; that value will be returned
    // out of the loop so you can use it

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn conditional_while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}
