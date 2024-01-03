fn main() {
    let v1: Vec<i32> = Vec::new();       // Vec with type i32
    println!("{:?}", v1);
    let mut v2 = vec![1, 2, 3];          // use vec! macro
    println!("{:?}", v2);

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);
    println!("v2 outside, {:?}", v2);

    {
        let v2 = vec![1, 2, 3, 4];
        println!("v2 inside,  {:?}", v2);
    }                                     // drop
    println!();

    let v3 = vec![1, 2, 3, 4, 5];
    
    let third: &i32 = &v3[2];
    println!("{}", third);
    let third: Option<&i32> = v3.get(2);  // get return enum
    println!("{:?}", third);

    // let does_not_exist = &v3[100];
    // println!("{}", does_not_exist);    // panic!
    let does_not_exist = v3.get(100);
    println!("{:?}", does_not_exist);     // return None
    
    let mut v4 = vec![1, 2, 3, 4, 5];
    let first = &v4[0];                   // borrow first as immutalbe
    v4.push(6);                           // mutable borrow, error
    // println!("{} / {:?}", first, v4);
    println!();

    let mut v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{}", i);
    }

    for i in &mut v5 {
        *i += 50;                         // use dereference(*) for change reference value
    }
    println!("{:?}", v5);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}
