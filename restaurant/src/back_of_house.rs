#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

pub struct Breakfirst {
    pub toast: String,
    season_fruit: String,
}

impl Breakfirst {
    pub fn summer(toast: &str) -> Breakfirst {
        Breakfirst {
            toast: String::from(toast),
            season_fruit: String::from("peaches"),
        }
    }
}

fn fix_incorrect_order() {
    cook_order();
    // use super to call parent module function
    super::delivery_order();
}

fn cook_order() {}
