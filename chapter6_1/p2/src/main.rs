#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn main () {
let ip_version_4 = IpAddr {
    kind:IpAddrKind::v4,
    address:String::from("127.0.0.1"),
};

let ip_version_6 = IpAddr {
    kind:IpAddrKind::v6,
    address:String::from("..1"),
};

println!("{:#?}",ip_version_4);
println!("{:#?}",ip_version_6);
}