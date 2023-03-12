use gui::Draw;
#[derive(Debug)]
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{:?}", self);
    }
}
fn main() {}
