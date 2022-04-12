struct User {
    //  we used the owned String type rather than the &str string slice type.
    //  This is a deliberate choice because we want each instance of this struct
    //  to own all of its data and for that data to be valid for as long as the
    //  entire struct is valid.

    // It’s also possible for structs to store references to data owned by
    // something else, but to do so requires the use of lifetimes
    // Lifetimes ensure that the data referenced by a struct is valid for as
    // long as the struct is.
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    //  Because the parameter names and the struct field names are exactly the
    //  same,  we can use the field init shorthand syntax to rewrite build_user
    //  so that it behaves exactly the same but doesn’t have the repetition of
    //  email and username

    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // Note that the entire instance must be mutable; Rust doesn’t allow us to
    // mark only certain fields as mutable.
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // It’s often useful to create a new instance of a struct that includes most
    // of the values from another instance, but changes some. You can do this
    // using struct update syntax.

    // Using struct update syntax, we can achieve the same effect with less
    // code, as shown in Listing 5-7. The syntax .. specifies that the remaining
    // fields not explicitly set should have the same value as the fields in the
    // given instance.
    // The ..user1 must come last to specify that any remaining fields should
    // get their values from the corresponding fields in user1, but we can
    // choose to specify values for as many fields as we want in any order,
    // regardless of the order of the fields in the struct’s definition.
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // we can no longer use user1 after creating user2 because the String in the
    // username field of user1 was moved into user2
}

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn unit_like_structs() {
    // You can also define structs that don’t have any fields! These are called
    // unit-like structs because they behave similarly to (), the unit type that
    // we mentioned in “The Tuple Type” section. Unit-like structs can be useful
    // when you need to implement a trait on some type but don’t have any data
    // that you want to store in the type itself.
    let subject = AlwaysEqual;
}
