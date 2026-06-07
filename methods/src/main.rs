#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// We can use multiple impl
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Like static method in other languages
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 1920,
        height: 960
    };
    println!("The area of rectangle is {} pixels", rect1.area());

    if rect1.width() {
        println!("Width is greater than 0 ({})", rect1.width);
    }

    let rect2 = Rectangle {
        width: 960,
        height: 480
    };

    let rect3 = Rectangle {
        width: 4000,
        height: 2000
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Associated functions
    let rect4 = Rectangle::square(4000);
    println!("The area of square is {} pixels", rect4.area());
}