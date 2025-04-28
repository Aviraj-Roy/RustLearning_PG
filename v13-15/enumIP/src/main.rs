#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    address: String,
    kind: IpAddrKind,
}

impl IpAddr {
    fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            kind: IpAddrKind::V4,
        }
    }
}

fn main() {
    /*let google_address = IpAddr {
        address: String::from("1.2.3.4"),
        kind: IpAddrKind::V4,
    };*/
    let google_address = IpAddr::new("1.2.3.4");
    let loopback = IpAddr::new("127.0.0.1");
    route(google_address);
}

fn route(ip: IpAddr) {
    println!("Routing request to IP {} of kind {:?}", ip.address, ip.kind);
}
