use std::{thread, time::Duration, vec};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }
    //Move using closures
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // Channels
}
