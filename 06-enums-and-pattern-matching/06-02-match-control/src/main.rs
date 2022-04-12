fn main() {
    println!("Hello, world!");

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));
    matching_with_option();
    plus_one_without_none();
    dice_roll_1();
    dice_roll_2();
    dice_roll_3();
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // long arm
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,

        // Another useful feature of match arms is that they can bind to the
        // parts of the values that match the pattern. This is how we can
        // extract values out of enum variants.
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn matching_with_option() {
    let five = Some(5);

    // Does Some(5) match Some(i)? Why yes it does! We have the same variant.
    // The i binds to the value contained in Some, so i takes the value 5. The
    // code in the match arm is then executed, so we add 1 to the value of i and
    // create a new Some value with our total 6 inside.
    let six = plus_one(five);

    let none = plus_one(None);

    // println!("{},{},{}", five, six, none);
}

fn matches_are_exhaustive() {
    // We didn’t handle the None case, so this code will cause a bug. Luckily,
    // it’s a bug Rust knows how to catch.

    plus_one_without_none();
}

fn plus_one_without_none(x: Option<i32>) -> Option<i32> {
    // Rust knows that we didn’t cover every possible case and even knows which
    // pattern we forgot! Matches in Rust are exhaustive: we must exhaust every
    // last possibility in order for the code to be valid. Especially in the
    // case of Option<T>, when Rust prevents us from forgetting to explicitly
    // handle the None case, it protects us from assuming that we have a value
    // when we might have null
    match x {
        Some(i) => Some(i + 1),
    }
}

fn dice_roll_1() {
    // Imagine we’re implementing a game where, if you roll a 3 on a dice roll,
    // your player doesn’t move, but instead gets a new fancy hat. If you roll a
    // 7, your player loses a fancy hat. For all other values, your player moves
    // that number of spaces on the game board.

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn dice_roll_2() {
    // Rust also has a pattern we can use when we don’t want to use the value in
    // the catch-all pattern: _, which is a special pattern that matches any
    // value and does not bind to that value. This tells Rust we aren’t going to
    // use the value, so Rust won’t warn us about an unused variable.

    // Let’s change the rules of the game to be that if you roll anything other
    // than a 3 or a 7, you must roll again. We don’t need to use the value in
    // that case, so we can change our code to use _ instead of the variable
    // named other
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

fn dice_roll_3() {
    // This example also meets the exhaustiveness requirement because we’re
    // explicitly ignoring all other values in the last arm; we haven’t
    // forgotten anything.

    //Here, we’re telling Rust explicitly that we aren’t going to use any other
    //value that doesn’t match a pattern in an earlier arm, and we don’t want to
    //run any code in this case.

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
