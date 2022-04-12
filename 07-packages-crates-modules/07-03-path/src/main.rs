// A path can take two forms:
// - An absolute path starts from a crate root by using a crate name or a literal crate.
// - A relative path starts from the current module and uses self, super, or an
//   identifier in the current module.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn main() {
    println!("Hello, world!");
}

fn serve_order() {}

mod back_of_house {
    // We can also construct relative paths that begin in the parent module by
    // using super at the start of the path. This is like starting a filesystem
    // path with the .. syntax.

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    // Because the toast field in the back_of_house::Breakfast struct is public,
    // in eat_at_restaurant we can write and read to the toast field using dot
    // notation.

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

//Because we made the Appetizer enum public, we can use the Soup and Salad
//variants in eat_at_restaurant. Enums aren’t very useful unless their variants
//are public; it would be annoying to have to annotate all enum variants with
//pub in every case, so the default for enum variants is to be public. Structs
//are often useful without their fields being public, so struct fields follow
//the general rule of everything being private by default unless annotated with
//pub.

mod back_of_house_1 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
