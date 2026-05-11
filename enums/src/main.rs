fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {}

    route(four);
    route(IpAddrKind::V4);

    // structs
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    };

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    enum IpAddrKind2 {
        V4(String),
        V6(String),
    };

    let home2 = IpAddrKind2::V4(String::from("127.0.0.1"));

    // enhanced enums
    enum IpAddrKind3 {
        V4(u8, u8, u8, u8),
        V6(String),
    };

    let home3 = IpAddrKind3::V4(127, 0, 0, 1);
}
