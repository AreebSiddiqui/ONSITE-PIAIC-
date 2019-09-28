#[derive(Debug)]
struct IpAddressV4 {
    kind:String,
}
#[derive(Debug)]
struct IpAddressV6 {
    kind:String,
}

impl IpAddressV4 {
    fn new (x: String) -> IpAddressV4 {
        IpAddressV4 {
            kind: x,
        }
    }
}

impl IpAddressV6 {
    fn new (x: String) -> IpAddressV6 {
        IpAddressV6 {
            kind: x,
        }
    }
}
#[derive(Debug)]
enum IpAddr {
    V4(IpAddressV4),
    V6(IpAddressV6),
}

fn main() {
    //variable     //enum //var //Struct //associated
    let version4 = IpAddr::V4(IpAddressV4::new(String::from("127.1.1.0")));
    let version6 = IpAddr::V6(IpAddressV6::new(String::from("0.0.127.1.1.0")));
    println!("{:#?}",version4);
    println!("{:#?}",version6);
}


