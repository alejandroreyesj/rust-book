#![allow(dead_code)]
fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    // Using tuples
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels. (Using tuples)",
        area2(rect1)
    );

    // Using structs
    let rect2 = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels. (Using struct)",
        area3(&rect2)
    );

    println!(
        "The area of the rectangle is {} square pixels. (Using methods)",
        rect2.area()
    );

    let rect3 = Rectangle {
        width: 20,
        height: 30,
    };

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    println!("Pretty print rect 2 using :#?: {rect2:#?}")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
