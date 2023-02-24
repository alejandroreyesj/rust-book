fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result1 = largest(&number_list);
    let result2 = largest(&number_list2);
    let char_result = largest_generic(&char_list);
    println!("The result1 number is {result1}");
    println!("The result2 number is {result2}");
    println!("The largest char is {char_result}");
}
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_generic<T: Ord + PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
