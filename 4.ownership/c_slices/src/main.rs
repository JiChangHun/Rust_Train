fn main() {
    let mut s1 = String::from("hello world!");
    let (word1, word2) = words(&s1);
    println!("words_bytes: {} .. {}", word1, word2);
    s1.clear();                                 // mutable borrow
    // println!("{}", word);                    // cannot borrow, also borrowed as immutable

    let s2 = String::from("hello world!");
    let hello = &s2[..5];
    let world = &s2[6..];
    println!("just slice: {}, {}", hello, world);
    println!();

    let s3 = String::from("hello world");
    let (word1, word2) = words(&s3[..]);
    println!("words_bytes slice: {} ... {}", word1, word2);

    let (word1, word2) = words_char(&s3);
    println!("words_char test: {} ... {}", word1, word2);

    let s3_literal = "hello world";                         // string literal is slice and type &str

    let (w1, w2) = words(&s3_literal[..]);
    println!("words literal: {} .... {}", w1, w2);

    let (w1, w2) = words(s3_literal);
    println!("words literal2: {} .... {}", w1, w2);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("array: {:?}", slice);
}

fn words(s: &str) -> (&str, &str) {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&s[..i], &s[i+1..]);
        }
    }
    (&s[..], "")
}

fn words_char(s: &String) -> (&str, &str) {
    for (i, item) in s.chars().enumerate() {
        if item == ' ' {
            return (&s[..i], &s[i+1..])
        }
    }
    (&s[..], "")
}