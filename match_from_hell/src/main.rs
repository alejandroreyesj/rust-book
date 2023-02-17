fn main() {
    let mut x = 42;

    let a = &mut x;
    println!("a = {a}");

    let b = &mut x;
    println!("b = {b}");
}
