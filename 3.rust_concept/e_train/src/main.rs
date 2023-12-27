fn main() {
    let d1 = 32;
    let option1 = "F to C";
    let result = change_fc(&d1, option1);
    println!("{}{} result {:?}", d1, option1, result);

    let d2 = -10;
    let option2 = "C to F";
    let result = change_fc(&d2, option2);
    println!("{}{} result {:?}", d2, option2, result);
    println!();

    let n = 7;
    let result = fibo(n);
    println!("{}th fibo is {}", n.to_string(), result);
    let n = 23;
    let result = fibo(n);
    println!("{}th fibo is {}", n.to_string(), result);
}

fn change_fc(degree: &i32, option: &str) -> i32 {
    if option == "F to C" {
        (degree - 32) * 5 / 9
    } else {
        degree * 9 /5 + 32
    }
}

fn fibo(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        let mut a = 1;
        let mut b = 1;
        for _ in 1..n-1 {
            let temp = a;
            a = b;
            b = b + temp;
        }
        b
    }
}