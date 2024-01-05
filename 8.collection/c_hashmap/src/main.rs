use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Insert key, value {:?}", scores);

    // Use collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("Use collect {:?}", scores);
    println!();

    // Insert value
    let field_name = "Favorite color".to_owned();
    let field_value = "Blue".to_owned();
    let mut map = HashMap::new();
    println!("{} / {}", field_name, field_value);
    map.insert(field_name, field_value);                // move ownership
    println!("{:?}", map);
    // println!("{} / {}", field_name, field_value);    // error
    println!();

    // Show each key, value
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwriting
    let mut scores = HashMap::new();
    scores.insert("Blue".to_owned(), 10);
    scores.insert("Blue".to_owned(), 25);
    println!("changed scores {:?}", scores);

    // Use entry
    scores.entry("Yellow".to_owned()).or_insert(50);
    scores.entry("Blue".to_owned()).or_insert(50);
    println!("use entry scores {:?}", scores);
    println!();

    // Update value based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);   // return mutable reference
        *count += 1;                                // dereference using the asterisk(*)
    }
    println!("{:?}", map);
}
