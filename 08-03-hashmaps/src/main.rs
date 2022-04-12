use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    inserting_if_key_has_no_value();
}

fn create_new_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // . This HashMap has keys of type String and values of type i32. Like
    // vectors, hash maps are homogeneous: all of the keys must have the same
    // type, and all of the values must have the same type.
}

fn generating_hashmap_using_iterators() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    // The type annotation HashMap<_, _> is needed here because it’s possible to
    // collect into many different data structures and Rust doesn’t know which
    // you want unless you specify. For the parameters for the key and value
    // types, however, we use underscores, and Rust can infer the types that the
    // hash map contains based on the types of the data in the vectors.
}

fn hashmaps_and_ownerships() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // For types that implement the Copy trait, like i32, the values are copied
    // into the hash map. For owned values like String, the values will be moved
    // and the hash map will be the owner of those values

    // If we insert references to values into the hash map, the values won’t be
    // moved into the hash map. The values that the references point to must be
    // valid for at least as long as the hash map is valid
    // unless lifetime is specified.
}

fn accessing_values_in_a_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Here, score will have the value that’s associated with the Blue team, and
    // the result will be Some(&10). The result is wrapped in Some because get
    // returns an Option<&V>; if there’s no value for that key in the hash map,
    // get will return None.
}

fn iterating_over_hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn overwriting_value() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

fn inserting_if_key_has_no_value() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:#?}", scores);
    // {"Blue": 10, "Yellow": 50}
}

fn updating_based_on_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // The split_whitespace method iterates over sub-slices, separated by
    // whitespace, of the value in text.

    for word in text.split_whitespace() {
        //  The or_insert method returns a mutable reference (&mut V) to the
        //  value for the specified key. Here we store that mutable reference in
        //  the count variable, so in order to assign to that value, we must
        //  first dereference count using the asterisk (*). The mutable
        //  reference goes out of scope at the end of the for loop, so all of
        //  these changes are safe and allowed by the borrowing rules.

        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
    // {"world": 2, "hello": 1, "wonderful": 1}
}
