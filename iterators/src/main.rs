fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    println!("{v1_iter:?}");

    println!("{:?}", v1_iter.map(|x| x * 2).sum::<i32>());
}
