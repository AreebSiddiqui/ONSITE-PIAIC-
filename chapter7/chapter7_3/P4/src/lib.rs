mod back_of_house {
    pub enum drinks {
        Black,
        White,
        Orange,
    }
}

pub fn drink_at_resturant(){
    let d1 = back_of_house::drinks::Black;
    let d2 = back_of_house::drinks::White;
}