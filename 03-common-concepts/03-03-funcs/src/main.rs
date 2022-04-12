fn main() {
    // Function bodies are made up of a series of statements optionally ending
    // in an expression.

    // * Statements are instructions that perform some action and do not return
    // * a value. Expressions evaluate to a resulting value.

    // Creating a variable and assigning a value to it with the let keyword is a
    // statement.

    // Expressions evaluate to a value and make up most of the rest of the code
    // that you’ll write in Rust. Consider a math operation, such as 5 + 6,
    // which is an expression that evaluates to the value 11.

    println!("Hello, world!");

    calling_a_func_is_an_expression();

    func_with_return_values();
}

fn calling_a_func_is_an_expression() {
    // {
    //     let x = 3;
    //     x + 1
    // }
    //
    // is a block that, in this case, evaluates to 4. That value gets bound to y
    // as part of the let statement. Note that the x + 1 line doesn’t have a
    // semicolon at the end, unlike most of the lines you’ve seen so far.
    // Expressions do not include ending semicolons. If you add a semicolon to
    // the end of an expression, you turn it into a statement, and it will then
    // not return a value.

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn func_with_return_values() {
    // You can return early from a function by using the return keyword and
    // specifying a value, but most functions return the last expression
    // implicitly.

    let x = five();

    println!("The value of x is: {}", x);
}
