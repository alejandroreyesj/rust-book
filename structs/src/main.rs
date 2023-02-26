#![allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someuser@example.com"),
        sing_in_count: 1,
    };
    println!("{user1:?}");
    user1.email = String::from("anothersomeuser@example.com");

    let user2 = User {
        email: String::from("someuser2@example.com"),
        ..user1
    };

    println!("{user2:?}");
    let color = Color(1, 2, 3);
    let point = Point(3, 2, 1);

    println!("Point: {}, {}, {}", point.0, point.1, point.2);
    println!("Color: {color:?}");
}
