fn main() {
    let _ = Solution::fizz_buzz(15);
}

struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|num| match (num % 3, num % 5) {
                (0, 0) => "FizzBuzz".to_string(),
                (0, _) => "Fizz".to_string(),
                (_, 0) => "Buzz".to_string(),
                (_, _) => num.to_string(),
            })
            .collect()
        // match (num % 3, num % 5) {
        //     (0,0) => answers.push("FizzBuzz".to_string()),
        //     (0, _) => answers.push("Fizz".to_string()),
        //     (_, 0) => answers.push("Buzz".to_string()),
        //     (_,_) => answers.push(format!("{num}"))
        // };
        // }
        // answers
    }
}
