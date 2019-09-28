enum Coin {
    OneRupee,
    TwoRupee,
    FiveRupee,
    TenRupee,
}

fn main() {
    let result = return_value(Coin::OneRupee);
    println!("{}",result);
}

fn return_value (c : Coin ) -> u8 {
    match c {
        Coin::OneRupee => { 
            println!("OneRupeeCoin");
            1 
        },
        Coin::TwoRupee => 2,
        Coin::FiveRupee => 5,
        Coin::TenRupee => 10,
    }
}
