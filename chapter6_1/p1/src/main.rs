#[derive(Debug)]
enum IpAddress {
    V4,
    V6
}

fn main() {
    let ip_version_4 = IpAddress::V4;
    let ip_version_6 = IpAddress::V6;

    route(ip_version_4);
    route(IpAddress::V6);

}

fn route(ipkind: IpAddress) {
   println!("{:#?}",ipkind);
}