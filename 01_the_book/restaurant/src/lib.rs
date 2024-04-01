fn deliver_order() {}

mod front_of_house;

pub use crate::front_of_house::hosting;

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
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    back_of_house::fix_incorrect_order();
    let meal = back_of_house::Breakfast::summer("Rye");

    println!("I'd like {} toast please", meal.toast);
}
