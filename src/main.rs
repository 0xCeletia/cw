fn main() {
    #[derive(Debug)]
    enum IpAddrKind { // Any IP address can be either a version four or a version six address, but not both at the same time. 
        V6,
        V4,
        V2,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let maison = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };


    enum IpAddr_ {
        V6(String),
        V4(String),
    }

    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Write(String),
        Move {x: i32, y: i32},
    }

    enum Option<T> { // <T> -> It’s a generic type parameter
/* each concrete type that gets used in place of T makes the overall Option<T> type a different type. */
        None,
        Some(T),
    }

    let _six = IpAddr_::V6(String::from("::1"));


    fn route(ip_kind: IpAddrKind) {}

    let four = IpAddrKind::V4;

    println!("{:#?}", &four);

    route(IpAddrKind::V6);
    route(IpAddrKind::V2);
}