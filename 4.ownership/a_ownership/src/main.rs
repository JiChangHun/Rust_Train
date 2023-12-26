fn main() {
    let s = "hello1";                    // String literal, immutable
    // s.push_str(", world!");           // error, method not found in '&str'
    println!("{}", s);

    let mut m = String::from("hello2");  // String, mutable
    m.push_str(", world!");
    println!("{}", m);

    {
        let s = String::from("hello3");  // allocate
        println!("{}", s);
    }                                    // drop, automatic (C++, RAII)

    println!();

    let x = 5;
    let y = x;
    println!("x= {}, y= {}", x, y);      // no error, copy fast, save to stack, have copy trait(int, bool, float)

    // prevent double free error, memory corruption
    let s1 = String::from("hello");      // stack(pointer, len:byte, capacity:byte), heap(index, value)
    let s2 = s1;                         // move(not shallow copy)
    // println!("{}", s1);               // error, value borrowed here after move
    println!("{}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();                 // deep copy
    println!("s1 = {}, s2 = {}", s1, s2);

    println!();

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);                // error, taked ownership

    let x = 5;
    makes_copy(x);
    println!("not taked x = {}", x);

    println!();

    let s1 = gives_ownership();
    println!("give ownership: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2.clone());
    println!("take and give ownership: {} / {}", s2, s3);

    println!();

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("ownership test String, {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("ownership test integer, {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(mut a_string: String) -> String {
    a_string.push_str(", world!");
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}