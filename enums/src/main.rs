#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // better way
    let home = IpAddrKindWithAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKindWithAddr::V6(String::from("::1"));

    // multiple type enums
    let m = Message::Write(String::from("hello"));
    m.call();
}

// enum and struct
enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// consise enum with data integrated
enum IpAddrKindWithAddr {
    V4(String),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {}
}
