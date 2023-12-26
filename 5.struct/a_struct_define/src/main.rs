fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    let user2 = build_user("email@email.com".to_string(), "name".to_string());
    println!("user2: {} / {} / {} / {}", user2.email, user2.username, user2.active, user2.sign_in_count);

    let user3 = User {
        email: String::from("another@exmaple.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("user3: {} / {} / {} / {}", user3.email, user3.username, user3.active, user3.sign_in_count);

    let user4 = User {
        email: String::from("another2@exmaple.com"),
        username: String::from("anotherusername678"),
        ..user1
    };
    println!("user4: {} / {} / {} / {}", user4.email, user4.username, user4.active, user4.sign_in_count);
    println!();

    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("color: {:?}, point: {:?}", black, origin);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
