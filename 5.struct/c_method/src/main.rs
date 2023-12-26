#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other_rec: &Rectangle) -> bool {
        self.length >= other_rec.length && self.width >= other_rec.width
    }
    // associated functions
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    println!(
        "The area of the rectangle is {} square pixels. (use method)",
        rect1.area()
    );

    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(40);
    let square2 = Rectangle::square(30);
    println!("Can rect1 hold square1? {}", rect1.can_hold(&square1));
    println!("Can rect1 hold square2? {}", rect1.can_hold(&square2));
}
