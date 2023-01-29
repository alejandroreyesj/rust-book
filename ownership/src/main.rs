fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    // String Slices
    let s = String::from("Wealthy World is Lost");
    println!("{}", first_word(&s));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
