fn main() {
    println!("Hello, world!");

    concise_control_flow_with_if_let();
    coin_example();
}

fn concise_control_flow_with_if_let() {
    // The if let syntax lets you combine if and let into a less verbose way to
    // handle values that match one pattern while ignoring the rest.

    // We don’t want to do anything with the None value, which is an annoying
    // boilerplat code to add in the normal match arm format

    // However, you lose the exhaustive checking that match enforces.
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn coin_example() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
