struct Fullname {
    first_name:String,
    middle_name:Option<String>,
    last_name: String,
}


fn main() {
    let qauid_e_azam = Fullname {
        first_name:String::from("Muhammad"),
        middle_name:Option::Some(String::from("Ali")),
        last_name:String::from("Jinnah"),
    };

    let shair_e_mashriq = Fullname {
        first_name:String::from("Allama"),
        middle_name:Option::None,
        last_name:String::from("Iqbal"),
    };

    let result = check(qauid_e_azam);
    println!("{}",result);
    let result = check(shair_e_mashriq);
    println!("{}",result);
    
}

fn check (name : Fullname) -> String {
    match name.middle_name {
        Some(i) => i,
        None => String::from("No middle name Found"),
    }
}
