#![allow(dead_code)]
#![allow(unused_variables)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn sit_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_the_house {
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

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // because we are importing hosting mod above, we can do the following:
    hosting::add_to_waitlist();

    let mut meal = back_of_the_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_the_house::Appetizer::Soup;
    let order2 = back_of_the_house::Appetizer::Salad;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
