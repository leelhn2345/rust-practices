// A trait tells the Rust compiler about functionality a particular type has and
// can share with other types. We can use traits to define shared behavior in an
// abstract way. We can use trait bounds to specify that a generic type can be
// any type that has certain behavior.

// ! source: https://doc.rust-lang.org/stable/book/ch10-02-traits.html

fn main() {
    println!("Hello, world!");
}

// ! defining a trait A type’s behavior consists of the methods we can call on
// that type. Different types share the same behavior if we can call the same
// methods on all of those types. Trait definitions are a way to group method
// signatures together to define a set of behaviors necessary to accomplish some
// purpose.

// For example, let’s say we have multiple structs that hold various kinds and
// amounts of text: a NewsArticle struct that holds a news story filed in a
// particular location and a Tweet that can have at most 280 characters along
// with metadata that indicates whether it was a new tweet, a retweet, or a
// reply to another tweet.

// We want to make a media aggregator library crate named aggregator that can
// display summaries of data that might be stored in a NewsArticle or Tweet
// instance. To do this, we need a summary from each type, and we’ll request
// that summary by calling a summarize method on an instance. Listing 10-12
// shows the definition of a public Summary trait that expresses this behavior.

pub trait Summary {
    fn summarize(&self) -> String;
}

// After the method signature, instead of providing an implementation within
// curly brackets, we use a semicolon. Each type implementing this trait must
// provide its own custom behavior for the body of the method. The compiler will
// enforce that any type that has the Summary trait will have the method
// summarize defined with this signature exactly. A trait can have multiple
// methods in its body: the method signatures are listed one per line and each
// line ends in a semicolon.

// ! implement a trait on type
// * the following is in src/lib.rs
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// * the following is in src/main.rs
// * have to bring TRAIT into scope Now that the library has implemented the
// Summary trait on NewsArticle and Tweet, users of the crate can call the trait
// methods on instances of NewsArticle and Tweet in the same way we call regular
// methods. The only difference is that the trait has to be brought into scope
// as well as the types to get the additional trait methods. Here’s an example
// of how a binary crate could use our aggregator library crate:

// use aggregator::{Summary, Tweet};

// fn main() { let tweet = Tweet { username: String::from("horse_ebooks"),
//     content: String::from( "of course, as you probably already know, people",
//         ), reply: false, retweet: false, };

//     println!("1 new tweet: {}", tweet.summarize());
// }

// ! default implementations Sometimes it’s useful to have default behavior for
// some or all of the methods in a trait instead of requiring implementations
// for all methods on every type. Then, as we implement the trait on a
// particular type, we can keep or override each method’s default behavior.

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}
// Even though we’re no longer defining the summarize method on NewsArticle
// directly, we’ve provided a default implementation and specified that
// NewsArticle implements the Summary trait.

fn default_trait_implementation() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

// This code prints New article available! (Read more...).

// ! default implementations calling other methods in same trait Default
// implementations can call other methods in the same trait, even if those other
// methods don’t have a default implementation. In this way, a trait can provide
// a lot of useful functionality and only require implementors to specify a
// small part of it.
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn trait_other_methods() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    // This code prints 1 new tweet: (Read more from @horse_ebooks...).
}

// ! traits as parameters We can define a notify function that calls the
// summarize method on its item parameter, which is of some type that implements
// the Summary trait. To do this, we can use the impl Trait syntax, like this:
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// ! trait bound syntax The impl Trait syntax works for straightforward cases
// but is actually syntax sugar for a longer form, which is called a trait
// bound; it looks like this:
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// * compare the following they are the same
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// ! specifying multiple trait bounds with the + syntax
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}

// ! clearer trait bounds with where clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}
// This function’s signature is less cluttered: the function name, parameter
// list, and return type are close together, similar to a function without lots
// of trait bounds.

// ! returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
// By using impl Summary for the return type, we specify that the
// returns_summarizable function returns some type that implements the Summary
// trait without naming the concrete type. In this case, returns_summarizable
// returns a Tweet, but the code calling this function doesn’t know that.
// # However, you can only use impl Trait if you’re returning a single type.
// using a if else flow in function that returns `impl Summary` wouldn't work

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // when we made the largest function generic, it became possible for the
    // list parameter to have types in it that don’t implement the Copy trait.

    // # If we don’t want to restrict the largest function to the types that
    // implement the Copy trait, we could specify that T has the trait bound
    // Clone instead of Copy. Then we could clone each value in the slice when
    // we want the largest function to have ownership. Using the clone function
    // means we’re potentially making more heap allocations in the case of types
    // that own heap data like String, and heap allocations can be slow if we’re
    // working with large amounts of data.

    // ! another way
    // Another way we could implement largest is for the function to return a
    // reference to a T value in the slice. If we change the return type to &T
    // instead of T, thereby changing the body of the function to return a
    // reference, we wouldn’t need the Clone or Copy trait bounds and we could
    // avoid heap allocations.

    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generic_largest_main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// ! using trait bounds to conditionally implement methods
// By using a trait bound with an impl block that uses generic type parameters,
// we can implement methods conditionally for types that implement the specified
// traits.

//  But in the next impl block, Pair<T> only implements the cmp_display method
//  if its inner type T implements the PartialOrd trait that enables comparison
//  and the Display trait that enables printing.
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// ! conditional trait implementation
// We can also conditionally implement a trait for any type that implements
// another trait.

// the standard library implements the ToString trait on any type that
// implements the Display trait.
impl<T: Display> ToString for T {
    // --snip--
}
// Because the standard library has this blanket implementation, we can call the
// to_string method defined by the ToString trait on any type that implements
// the Display trait.
let s = 3.to_string();

// ! summary
// Traits and trait bounds let us write code that uses generic type parameters
// to reduce duplication but also specify to the compiler that we want the
// generic type to have particular behavior. The compiler can then use the trait
// bound information to check that all the concrete types used with our code
// provide the correct behavior. 
