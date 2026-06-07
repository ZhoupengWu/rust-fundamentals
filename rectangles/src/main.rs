#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    /* let width1 = 1920;
    let height1 = 960; */
    /* let rect1 = (1920, 960); */
    let rect1 = Rectangle {
        width: 1920,
        height: 960
    };
    println!("The area is  {} pixels", area(&rect1));
    println!("Rect1: {rect1:?}");
    println!("Rect1: {rect1:#?}");
    dbg!(&rect1);
}

/* fn area(width: u32, height: u32) -> u32 {
    width * height
} */

/* fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
} */

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}