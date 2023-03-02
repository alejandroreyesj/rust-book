use add_one::add_one;
use add_two::add_two;
fn main() {
    let x = 5;
    let x = add_one(x);
    let x = add_two(x);
    println!("#{x:?}");
}
