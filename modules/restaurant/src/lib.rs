mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

use std::{fmt, io};

fn function1() -> fmt::Result {}

fn function2() -> io::Result {}
