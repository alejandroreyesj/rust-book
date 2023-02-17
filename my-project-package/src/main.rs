pub mod add;

use crate::add::add;
fn main() {
    let x = 4;
    let y = 4;
    let z = add(x, y);
    println!("{z}");
}
