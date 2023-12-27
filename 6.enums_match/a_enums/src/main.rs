#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn test_enum(ip_types: IpAddrKind) -> IpAddrKind { 
    ip_types
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// new with type
#[derive(Debug)]
enum IpAddrNew {
    V4(String),
    V6(String),
}

// new with V4
#[derive(Debug)]
enum IpAddrNew2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // test 
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let four = test_enum(four);
    let six = test_enum(six);
    println!("Test Ip types, {:?}", four);
    println!("Test Ip types, {:?}", six);
    println!();

    // use enum, with struct
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // use enum, without struct
    let home = IpAddrNew::V4(String::from("127.0.0.1"));
    println!("home new1 {:?}", home);
    let loopback = IpAddrNew::V6(String::from("::1"));
    println!("loopback new1 {:?}", loopback);

    // use enum, without struct, tuple type
    let home = IpAddrNew2::V4(127, 0, 0, 1);
    println!("home new2 {:?}", home);
    let loopback = IpAddrNew2::V6(String::from("::1"));
    println!("loopback new2 {:?}", loopback);
    
    // print each value
    if let IpAddrNew2::V4(a, b, c, d) = home {
        println!("The IP is {}/{}/{}/{}", a, b, c, d);
    }
    println!();


    // message, call
    let m = Message::Write(String::from("hello"));
    println!("method call {}", m.call());

    // Option
    let some_number = Some(5);
    let some_string = Some("a string");
    println!("{:?} / {:?}", some_number, some_string);

    let absent_number: Option<i32> = None;
    println!("{:?}", absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let z: Option<i8> = Some(6);
    // let sum = y + z;                    // cannot add option to option
    // let sum = x + y;                    // different types, need to change option<T> to T
    let sum = x + y.unwrap();
    println!("{}", sum);
    let sum = y.unwrap() + z.unwrap();
    println!("{}", sum);
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> i32 {
        1
    }
}