enum IpAddrKind {
    V4,
    V6,
}

enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    fn new(kind: IpAddrKind, address: String) -> Self {
        Self { kind, address }
    }
}

fn main() {
    let home_og = IpAddr::new(IpAddrKind::V4, String::from("127.0.0.1"));
    let _loopback = IpAddr::new(IpAddrKind::V6, String::from("::1"));

    let _home_address = home_og.address;
    let _home_address_type = home_og.kind;

    let _home = IpAddress::V4(127, 0, 0, 1);
    let _loopback = IpAddress::V6(String::from("::1"));
}
