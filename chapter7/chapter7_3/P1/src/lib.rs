// mod customer_expericence {}
mod front_house {
     pub mod hosting {
         pub fn add_to_waitlist() {

         }
     }
}
//mod dining {}
pub fn eat_at_resturant () {
    //Absolute Path
    crate::front_house::hosting::add_to_waitlist();
    //Relative Path
    front_house::hosting::add_to_waitlist();
}
