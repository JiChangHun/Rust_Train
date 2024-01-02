// nested_module
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}
use a::series::of::nested_modules;

// enum
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
use TrafficLight::{Red, Yellow};

// or use glob
#[derive(Debug)]
enum TrafficLight2 {
    Red2,
    Yellow2,
    Green2,
}
use TrafficLight2::*;   // can get naming conflict

fn main() {
    // nested_module
    nested_modules();
    // enum
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
    println!("{:?} / {:?} / {:?}", red, yellow, green);
    // use glob
    let red = Red2;
    let yellow = Yellow2;
    let green = Green2;
    println!("{:?} / {:?} / {:?}", red, yellow, green);
}