pub mod front_house;

pub use crate::front_house::hosting;
pub fn eat_at_resturant() {
    hosting::add_to_waitlist();
}
