pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {
            println!("Add to wait list.");
        }

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn server_order() {}

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

            fn cook_order() {}

            fn fix_incorrect_order() {
                cook_order();
                super::server_order();
            }
        }

        fn take_payment() {}

        pub fn eat_at_restaurant(toast: &str) -> String {
            // Order a breakfast in the summer with Rye toast
            let mut meal = back_of_house::Breakfast::summer("Rye");
            // Change our mind about what bread we'd like
            meal.toast = String::from(toast);
            return String::from(&meal.toast);
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::day5::restaurant::front_of_house::hosting::add_to_wait_list();
    // Relative path
    front_of_house::hosting::add_to_wait_list();
}
