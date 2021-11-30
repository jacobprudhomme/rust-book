enum IpAddrKind {
    V4,
    V6,
}

// We can use enums in combination with structs
struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

// More conveniently, we can use data directly in each enum variant
enum IpAddr2 {
    // V4(String),
    V4(u8, u8, u8, u8),  // Each variant can take in different data (cannot do this with a struct!)
    V6(String),
}

// The definition of IpAddr in the standard library shows we can use any type of data inside an enum variant
// struct Ipv4Addr {
//     ...
// }
//
// struct Ipv6Addr {
//     ...
// }
//
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },  // We can let a variant have named fields, just like a struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

// We can define methods on enums too
impl Message {
    fn call(&self) {}
}

// The standard library offers Option<T>, an enum for the presence or absence of a value (like null). Here, T is a generic type parameter
// It's better than null because every non-Option type must be a fully realized value, and any Option type needs to handle the case where it is None before it can be unwrapped to get the (potential) inner value
// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
    // Enums are namespaced under their identifier
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // All variants are of the same type (IpAddrKind)
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // These variant constructors are automatically generated
    // let home = IpAddr2::V4(String::from("127.0.0.1"));
    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    let some_number = Some(5);  // : Option<i32>
    let some_string = Some("a string");  // : Option<&str>
    let absent_number: Option<i32> = None;  // The compiler can't infer the type because None doesn't hold any data, so we need to annotate it
}

fn route(ip_kind: IpAddrKind) {}
