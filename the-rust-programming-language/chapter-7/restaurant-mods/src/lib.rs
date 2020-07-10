mod back_of_house;
mod front_of_house;

// pub use crate::back_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // because we are importing hosting mod above, we can do the following:
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::eat_at_restaurant();
    }
}
