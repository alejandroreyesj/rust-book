mod blog;
mod rust_blog;
use blog::Post;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // State Pattern
    // We define a set of states
    // The states are represented by state objects.
    // A value's bahavior changes based on its state

    // OOP State Pattern
    let mut post = rust_blog::Post::new();

    post.add_text("I ate a salad for lunch today");

    let mut post = post.request_review();
    let post = post.approve();
    match post {
        Some(post) => {
            println!("Post: {}", post.content());
        }
        None => {
            println!("Post is not approved");
        }
    }
    println!("DONE");

    // idiomatic Rust State Pattern
    // let mut post = rust_blog::Post::new();
    // post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());
    Ok(())
}
