fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(5);
    another_function3(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is: {} and the value of y is: {}", x, y);

    let f = five();
    println!("The value of f is {f}");

    let six = plus_one(f);
    println!("The value of six is {six}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("The value of x is: {} and the value of y is: {}", x, y);
    println!();
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}