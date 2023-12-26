fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectanble is {} square pixels.",
        area(length1, width1)
    );


    let rect1 = (50, 30);
    println!(
        "The area of the rectanble is {} square pixels. (use tuple)",
        area_tuple(rect1)
    );

    let rect2 = Rectangle { length: 50, width: 30 };
    println!(
        "The area of the rectangle is {} square pixels. (use struct)",
        area_struct(rect2)
    );
    // let rect2_copy = rect2;                      // cannot use because of no ownership

    let rect3 = Rectangle { length: 50, width: 30 };
    println!(
        "The area of the rectangle is {} square pixels. (use struct_refer)",
        area_struct_refer(&rect3)
    );
    println!("The rect struct is {:#?}", rect3);    // use Debug print format
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]                                    // add derive annotation for print 
struct Rectangle {
    length: u32,
    width: u32,
}

fn area_struct(rectangle: Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

fn area_struct_refer(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}