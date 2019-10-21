// #![allow(dead_code)]
// mod front_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         pub fn seat_at_table() {}
//     }

//     pub mod serving {
//         pub fn take_order() {}
//         pub fn serve_order() {}
//         pub fn take_payment() {} 
//     }
// }

//  pub mod front_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {

//         }
//     } 
// }

// mod dining {
// pub fn eat_at_resturant() {
//     //Absolute path 
//     crate::front_house::hosting::add_to_waitlist(); //this path get wrong
//     //Relative Path
//     front_house::hosting::add_to_waitlist();
// }
// }



// pub fn serve_order() {}

// pub mod back_of_house{
//     pub fn deliver() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }


// pub mod back_of_house {
//     #[derive(Debug)]
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn new (toast: String) -> Breakfast {
//                 Breakfast{
//                 toast,
//                 seasonal_fruit:String::from("peaches"),
//             }
//         }
//     }
// }


// pub fn eat_at_resturant () {
//     let mut meal = back_of_house::Breakfast::new(String::from("Wheat")); //By deafult
//     println!("{:#?}",meal);
// }

// pub mod back_of_house {
//     pub enum appetizer {
//         Salad,
//         Soup,
//     }

// }

// pub fn eat_at_resturant() {
//     back_of_house::appetizer::Salad;
//     back_of_house::appetizer::Soup;
// }



pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist () {

        }
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_resturant () {

    hosting::add_to_waitlist();

}