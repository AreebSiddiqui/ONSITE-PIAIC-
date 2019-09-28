fn main () {
    let some_val = 3;
    position(some_val);
}

fn position (x: u32) {
    match x {
        1 => println!("Frist Position"),
        2 => println!("Second Position"),
        _ => println!("{} is not a Position",x),
    }

}
