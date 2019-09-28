#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    email: String,
    country: String,

}
#[derive(Debug)]
enum Months {
    Jan,
    Feb,
    March,
    April,
}

fn main () {
    let user1 = User {
        name:String::from("Ali"),
        age:25,
        email:String::from("Ali@gmail.com"),
        country:String::from("Pakistan"),
    };

    println!("{:#?}",user1);
    let J = Months::Jan;
    println!("{:#?}",J);
}

