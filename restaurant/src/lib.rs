mod back_of_house;
mod customer;
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn delivery_order() {}

pub fn eat_at_restaurant() {
    hosting::add_wait_list();

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("{:?}, {:?}", order1, order2);

    let mut meal = back_of_house::Breakfirst::summer("Rye");
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
