fn main() {
    println!("Hello, world!");
    create_vector();
    updating_vector();
}

fn create_vector() {
    // Note that we added a type annotation here. Because we aren’t inserting
    // any values into this vector, Rust doesn’t know what kind of elements we
    // intend to store
    let v: Vec<i32> = Vec::new();
    // More often, you’ll create a Vec<T> with initial values and Rust will
    // infer the type of value you want to store, so you rarely need to do this
    // type annotation. Rust conveniently provides the vec! macro, which will
    // create a new vector that holds the values you give it.
    let v = vec![1, 2, 3];
}

fn updating_vector() {
    // As with any variable, if we want to be able to change its value, we need
    // to make it mutable using the mut keyword, as discussed in Chapter 3. The
    // numbers we place inside are all of type i32, and Rust infers this from
    // the data, so we don’t need the Vec<i32> annotation.
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn reading_elements_of_vectors() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // Note two details here. First, we use the index value of 2 to get the
    // third element because vectors are indexed by number, starting at zero.
    // Second, we get the third element by either using & and [], which gives us
    // a reference, or using the get method with the index passed as an
    // argument, which gives us an Option<&T>.

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    // When we run this code, the first [] method will cause the program to
    // panic because it references a nonexistent element. This method is best
    // used when you want your program to crash if there’s an attempt to access
    // an element past the end of the vector.

    // When the get method is passed an index that is outside the vector, it returns
    // None without panicking. You would use this method if accessing an element
    // beyond the range of the vector may happen occasionally under normal
    // circumstances. Your code will then have logic to handle having either
    // Some(&element) or None
}

fn mutability_and_immutability() {
    // this code will produce compilation error
    // This error is due to the way vectors work: because vectors put the values
    // next to each other in memory, adding a new element onto the end of the
    // vector might require allocating new memory and copying the old elements
    // to the new space, if there isn’t enough room to put all the elements next
    // to each other where the vector is currently stored. In that case, the
    // reference to the first element would be pointing to deallocated memory.

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // immutable borrow occurs here

    v.push(6); // mutable borrow occurs here

    println!("The first element is: {}", first);
}

fn iterating_values_in_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

fn changing_values_in_vector() {
    // To change the value that the mutable reference refers to, we have to use
    // the * dereference operator to get to the value in i before we can use the
    // += operator

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

fn using_enum_in_vectors() {
    // Vectors can only store values that are the same type. This can be
    // inconvenient; there are definitely use cases for needing to store a list
    // of items of different types. Fortunately, the variants of an enum are
    // defined under the same enum type, so when we need one type to represent
    // elements of different types, we can define and use an enum!
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // Rust needs to know what types will be in the vector at compile time so it
    // knows exactly how much memory on the heap will be needed to store each
    // element. We must also be explicit about what types are allowed in this
    // vector. If Rust allowed a vector to hold any type, there would be a
    // chance that one or more of the types would cause errors with the
    // operations performed on the elements of the vector. Using an enum plus a
    // match expression means that Rust will ensure at compile time that every
    // possible case is handled

    // If you don’t know the exhaustive set of types a program will get at
    // runtime to store in a vector, the enum technique won’t work. Instead, you
    // can use a trait object
}
