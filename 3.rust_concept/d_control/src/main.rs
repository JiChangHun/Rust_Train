fn main() {
    let number = 3;
    check_condition(number);
    let number = 7;
    check_condition(number);
    println!();

    let number = 6;
    check_divisible(number);
    let number = 37;
    check_divisible(number);

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {number}");
    println!();

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    println!();

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    }
}

fn check_condition(num: i32) {
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn check_divisible(num: i32) {
    println!("number is {num}");
    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    println!();
}