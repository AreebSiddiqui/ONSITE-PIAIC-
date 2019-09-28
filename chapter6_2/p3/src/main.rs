fn main () {
let seven = Option::Some(7);    
let eight = plus_one(seven);
let absent = plus_one(None);
println!("{:#?}\n{:#?}",eight,absent);
}

fn plus_one (x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some (i+1),
    }
}