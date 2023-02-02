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

    println!("Pretty print rect 2 using :#?: {rect2:#?}")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
