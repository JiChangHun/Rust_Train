fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    let x = 2.0;        // f64
    let y: f32 = 3.0;   // f32
    println!("{x}, {y}");
    println!();

    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("{sum} / {diff} / {product} / {quotient} / {remainder}");

    let t = true;
    let f: bool = false;
    println!("{t} or {f}");
    println!();

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{c} / {z} / {heart_eyed_cat}");
    println!();

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup1 = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{tup:?}, {tup1:?}");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!();

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{five_hundred} / {six_point_four} / {one}");

    let a = [1, 2, 3, 4, 5];
    println!("{a:?}");
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    println!("{months:?}");
    println!("January is {} and February is {{months[1]}}", months[0]);
    println!();
}
