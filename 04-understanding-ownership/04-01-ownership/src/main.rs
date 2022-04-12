fn main() {
    // Data with an unknown size at compile time or a size that might change
    // must be stored on the heap instead.

    // The heap is less organized: when you put data on the heap, you request a
    // certain amount of space. The memory allocator finds an empty spot in the
    // heap that is big enough, marks it as being in use, and returns a pointer,
    // which is the address of that location. This process is called allocating
    // on the heap and is sometimes abbreviated as just allocating.
    let s = "hello";

    let s = String::from("hello");

    scope();
    data_on_stack();
    data_on_heap();

    println!("Hello, world!");
}

fn scope() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no
      // longer valid
}

fn data_on_stack() {
    // bind the value 5 to x; then make a copy of the value in x and bind it to
    // y.” We now have two variables, x and y, and both equal 5

    let x = 5;
    let y = x;
}

fn data_on_heap() {
    // When we assign s1 to s2, the String data is copied, meaning we copy the
    // pointer, the length, and the capacity that are on the stack. We do not
    // copy the data on the heap that the pointer refers to.

    let s1 = String::from("hello");
    let s2 = s1;

    //  when s2 and s1 go out of scope, they will both try to free the same
    //  memory. This is known as a double free error

    // To ensure memory safety, after the line let s2 = s1, Rust considers s1 as
    // no longer valid. Therefore, Rust doesn’t need to free anything when s1
    // goes out of scope. we would say that s1 was moved into s2

    println!("{}, world!", s1);
}

fn clone_data() {
    // This works just fine and explicitly produces the behavior shown in Figure
    // 4-3, where the heap data does get copied.

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn copy_data_on_stack() {
    // this code seems to contradict data_on_heap, but it works!
    // The reason is that types such as integers that have a known size at
    // compile time are stored entirely on the stack, so copies of the actual
    // values are quick to make.

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
