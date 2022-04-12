fn main() {
    println!("Hello, world!");
    // accessing individual characters in string is not supported in rust due to
    // utf-8 encoding

    // accessing the char value would only return byte value, `104` instead of
    // `h`

    // Indexing into a string is often a bad idea because it’s not clear what
    // the return type of the string-indexing operation should be: a byte value,
    // a character, a grapheme cluster, or a string slice.
}

fn creating_new_string() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // remember that strings are UTF-8 encoded

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn updating_string() {
    // The push_str method takes a string slice because we don’t necessarily
    // want to take ownership of the parameter.

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}

fn string_concatenation() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    //  String concatenation appends the string on the right to the string on
    //  the left and may require reallocation. This requires ownership of the
    //  string on the left
    let s3 = s1 + &s2;
    println!("{}", s3);

    // The reason s1 is no longer valid after the addition, and the reason we
    // used a reference to s2, has to do with the signature of the method that’s
    // called when we use the + operator. The + operator uses the add method,
    // whose signature looks something like this:
    //
    // fn add(self, s: &str) -> String {
}

fn string_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    // At this point, s will be tic-tac-toe
    // easier to see than: let s = s1 + "-" + &s2 + "-" + &s3;

    //  The format! macro works like println!, but instead of printing the
    //  output to the screen, it returns a String with the contents. The version
    //  of the code using format! is much easier to read, and the code generated
    //  by the format! macro uses references so that this call doesn’t take
    //  ownership of any of its parameters.
}

fn iterating_over_strings() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}
