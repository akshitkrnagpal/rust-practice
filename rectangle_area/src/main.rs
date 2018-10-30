struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rectangle = Rectangle { height: 4, width: 6 };

    println!("Area of Rectangle is {}", rectangle.area());
}
