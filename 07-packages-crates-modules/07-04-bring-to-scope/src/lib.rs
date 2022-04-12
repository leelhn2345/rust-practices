// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
use std::io::{self, Write};

use self::front_of_house::hosting;
// use crate::front_of_house::hosting;

// ! re-exporting names with pub use
pub use crate::front_of_house::hosting;

// * By using pub use, external code can now call the add_to_waitlist function
// * using hosting::add_to_waitlist.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// providing new names with the 'as' keyword
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
