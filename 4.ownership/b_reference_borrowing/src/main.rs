fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);                    // references, &
    println!("The length of '{}' is {}.", s1, len);     // can use s1

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("The value of s2 is {}", s2);
    
    let mut s3 = String::from("hello");
    let mut _r1 = &mut s3;
    let mut _r2 = &mut s3;                  // can't borrow mutable more than once at a time
    // change(&mut _r1);
    // change(&mut _r2); 

    let _r1 = &mut s3;
    {                                       // use scope
        let _r2 = &mut s3;
    }

    let mut s4 = String::from("hello");
    let _r1 = &s4;
    let _r2 = &s4;
    let mut _r3 = &mut s4;                  // can't mutable because it is also borrowed as immutable
    // println!("{}", _r1);
    println!();

    let reference_to_nothing = dangle();    // no value for it to be borrowed from
    let reference_to_string = not_dangle();
}

fn calculate_length(s: &String) -> usize {  // references, &
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");        // change, mutable references
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s                                      // missing lifetime specifier
}

fn not_dangle() -> String {
    let s = String::from("hello");
    s
}