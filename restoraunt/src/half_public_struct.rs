fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Pipe"),
            }
        }
    }
}


pub fn eat_at_restaraunt() {
    // order breakfast with rye break
    let mut meal = back_of_house::Breakfast::summer("rye");
    
    // change opinion about what toast you want
    meal.toast = String::from("wheat");

    println!("I am wonna {} tost, please", meal.toast);

    // we haven't access to view and change season fruits
    // meal.seasonal_fruit = String::from("blackberry");
}

