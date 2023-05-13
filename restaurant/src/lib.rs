mod front_of_house {
    pub mod hosting {
        pub fn add_wait_list() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn delivery_order() {}

mod back_of_hosue {
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
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_wait_list();
    front_of_house::hosting::add_wait_list();

    let order1 = back_of_hosue::Appetizer::Soup;
    let order2 = back_of_hosue::Appetizer::Salad;
    println!("{:?}, {:?}", order1, order2);

    let mut meal = back_of_hosue::Breakfirst::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // error[E0616]: field `season_fruit` of struct `Breakfirst` is private
    // meal.season_fruit = String::from("blueberries");
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
