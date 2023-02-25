pub mod generics_part_2_traits;
use std::fmt::Display;

use generics_part_2_traits::{NewsArticle, Summary};
fn main() {
    // Generics Part 1
    let number_list = vec![34, 50, 25, 100, 65];
    let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result1 = largest(&number_list);
    let result2 = largest(&number_list2);
    let char_result = largest_generic(&char_list);
    println!("The result1 number is {result1}");
    println!("The result2 number is {result2}");
    println!("The largest char is {char_result}");
    let article = NewsArticle::default();
    println!("{}", article.summarize());

    // Generics Part 3: Lifetimes;
    let string1 = String::from("abcde");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {result}");
    }
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Generics Part 1
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

// Liftimes on Structs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&self, announcement: &'b str) -> &'b str {
        println!("Attention please: {announcement}");
        announcement
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
