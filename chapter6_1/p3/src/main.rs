#[derive(Debug)]
enum IpKind {
    V4 (String),
    V6 (String),
}



fn main() {
    let ip_version_4 = IpKind::V4(String::from("127.0.0.1"));
    let ip_version_6 = IpKind::V6(String::from("..1"));
    println!("{:#?}",ip_version_4);
    println!("{:#?}",ip_version_6);
}


