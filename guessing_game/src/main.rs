use rand::Rng;
use std::io;

fn main() {
    println!("Guess te number!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_num) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too big!"),
        }
    }
}
