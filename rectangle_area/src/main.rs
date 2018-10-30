struct Rectangle {
    height: u32,
    width: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}


fn main() {
    let rectangle = Rectangle { height: 4, width: 6 };

    println!("Area of Rectangle is {}", area(&rectangle));
}
