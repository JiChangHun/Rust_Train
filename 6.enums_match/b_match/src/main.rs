#[derive(Debug)]
enum UsState {
    Alabmama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let value = value_in_cents(Coin::Penny);
    println!("{}", value);

    let value2 = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", value2);
    println!();

    // option match
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{} / {} / {:?}", five.unwrap(), six.unwrap(), none);

    // placeholder(_)
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    println!("{}", some_u8_value);
    println!();

    // if let
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    println!("{:?}", some_u8_value);

    // short
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;
    // match
    let coin = Coin::Quarter(UsState::Alaska);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!();


    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        println!("No state quarter");
        count += 1;
    }
    println!("The count is {}", count);

}