use std::collections::HashMap;

fn main() {
    // Vectors
    let v: Vec<i32> = vec![1, 1, 1, 1, 1, 1];
    println!("{v:?}");

    let third = &v[2];
    println!("{third}");

    let third_get = v.get(2);
    match third_get {
        Some(i) => println!("{i}"),
        None => println!("Index not found"),
    };

    //String
    let hindi = String::from("नमस्ते");
    let hindi_bytes = hindi.as_bytes();
    let hindi_chars: Vec<char> = hindi.chars().collect();
    println!("{hindi}");
    println!("{hindi_bytes:?}");
    println!("{hindi_chars:?}");

    //Hashmap
    let mut map = HashMap::new();
    map.insert(1, String::from("Blue"));
    map.insert(11, String::from("Red"));

    for (num, color) in &map {
        println!("{num}: {color}");
    }
}
