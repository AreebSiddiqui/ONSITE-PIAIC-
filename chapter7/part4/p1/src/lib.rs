pub mod front_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("hello world");
        }
    }
}
pub use crate::front_house::hosting;
pub fn eat_at_restuarant () {
    hosting::add_to_waitlist();
}