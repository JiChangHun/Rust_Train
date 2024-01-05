fn main() {
    let mut s = String::new();
    s.push_str("test");
    println!("test String s: {}", s);

    let data = "initial contents";
    println!("The data is {}", data);

    // the method also works on a literal directly
    let s = "initial contents".to_string();
    println!("to_string s is {}", s);
    // to_string, String::from same
    let s = String::from("initial contents");
    println!("String::from s is {}", s);
    println!();

    // can use any encoding data
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s1 is {}, s2 is {}", s1, s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;                          // add method, deref coercion, &String to &str(&s2[..])
    // println!("s1 is {}", s1);                // borrow of moved value
    println!("s2 is {}, s3 is {}", s2, s3);
    // let s3 = &s1 + &s2;                      // cannot add &String to &String
    println!();

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("Add result is {}", s);

    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);     // not take ownership
    println!("Format result is {} and s1 is {}", s, s1);
    println!();

    let _s1 = String::from("hello");
    // let h = _s1[0];                            // String cannot be indexed

    let len = String::from("Здравствуйте").len();
    println!("{}", len);                          // encoded by UTF-8, unicode scalar value, 2bytes each
    
    // Grapheme cluster
    println!("{}", "नमस्ते".to_string().len());     // saved u8 values as Vec
    // have six char, but fourth, sixth is diacritic(발음 구별 부호)

    // Slicing String
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("slice hello is {}", s);
    println!();

    let hello = "नमस्ते";
    // let s = &hello[0..4];                      // panic, 3bytes each

    for c in hello.chars() {
        print!("{} / ", c);
    }
    println!();

    for b in hello.bytes() {
        print!("{} / ", b);
    }
}
