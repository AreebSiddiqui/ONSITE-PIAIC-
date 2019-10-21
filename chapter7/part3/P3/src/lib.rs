mod front_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
       pub fn new (toast:String) -> Breakfast {
                Breakfast {
                toast,
                seasonal_fruit:String::from("Oranges"),
            }
        }
    }

}

 fn eat_at_restuarant () {
    let mut meal = front_house::Breakfast::new(String::from("Wheat"));
    meal.toast = String::from("Barlay");
    println!("{:#?}",meal.toast);
    meal.seasonal_fruit = String::from("peaches");
}