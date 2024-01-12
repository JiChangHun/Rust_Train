fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> {    // if using same generic types, two values must be same type
    x: T,
    y: U,
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointSame<T> {
    x: T,
    y: T,
}

impl<T> PointSame<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

struct PointXY<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointXY<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointXY<X2, Y2>) -> PointXY<X1, Y2> {
        PointXY {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // generic func
    let result = largest(&number_list);
    println!("Using Generic, The largest number is {}", result);
    let result = largest(&char_list);
    println!("Using Generic, The largest char is {}", result);
    println!();

    // generic struct
    let integer_and_float = Point { x: 5, y: 4.2 };
    println!("integer {}, float {}", integer_and_float.x, integer_and_float.y);

    let p = PointSame { x: 5, y: 10 };
    println!("p.x = {}, p.y = {}", p.x(), p.y());

    let float_and_float = Point { x: 5.5, y: 4.2 };
    println!("The distance is {}", float_and_float.distance_from_origin());

    let p1 = PointXY { x: 5, y: 10.4};
    let p2 = PointXY { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!();

    let _integer = OptionI32::Some(5);
    let _float = OptionF64::Some(5.0);
}

// monomorphization
enum OptionI32 {
    Some(i32),
    None,
}
enum OptionF64 {
    Some(f64),
    None,
}
