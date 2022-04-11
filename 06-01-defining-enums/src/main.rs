enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

#[derive(Debug)]
enum IpAddr {
    // each variant can have different types and amounts of associated data
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?},{:?}", home, loopback)

    let m = Message::Write(String::from("hello"));
    m.call();

}

fn option_num(){
    // Rust can infer these types because we’ve specified a value inside the
    // Some variant.
    let some_number = Some(5);
    let some_string = Some("a string");


    // For absent_number, Rust requires us to annotate the overall Option type:
    // the compiler can’t infer the type that the corresponding Some variant
    // will hold by looking only at a None value.
    let absent_number: Option<i32> = None;
    

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    //  Option<T> and T (where T can be any type) are different types, the
    //  compiler won’t let us use an Option<T> value as if it were definitely a
    //  valid value. 
    //  Rust doesn’t understand how to add an i8 and an Option<i8>, because
    //  they’re different types. 
    let sum = x + y;

    // In other words, you have to convert an Option<T> to a T before you can
    // perform T operations with it.
}
