use std::vec;

use average::AveragedCollection;

mod average;
mod excel_example;
fn main() {
    // Example of Encapsulation
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut avg = AveragedCollection::new(list);
    println!("{}", avg.average());
    avg.add(34);
    println!("{}", avg.average());
    avg.remove();
    println!("{}", avg.average());

    // Example of Trait Objects
}
