use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main () {

let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));


println!("{}",localhost_v4);

}
