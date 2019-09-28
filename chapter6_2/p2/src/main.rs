#![allow(dead_code)]
#[derive(Debug)]
enum Coin {
    OneRupee,
    TwoRupee,
    FiveRupee,
    TenRupee(PakStates),
}
#[derive(Debug)]
enum PakStates {
    Sindh,
    Punjab,
    Balochistan,
    KPK,
}

fn return_val_state (c : Coin) -> u8 {
    match c {
        Coin::OneRupee => 1,
        Coin::TwoRupee => 2,
        Coin::FiveRupee => 5,
        Coin::TenRupee(state) => {
            println!("TenRupee used in {:#?}",state);
            10
        },
    }

}

fn main () {
    let result = return_val_state(Coin::TenRupee(PakStates::Balochistan));
    println!("{:#?}",result);

}