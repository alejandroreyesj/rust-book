use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    // Example 1 Channels
    // thread::spawn(move || {
    //     let val = String::from("hello");
    //     tx.send(val).unwrap();
    //     // println!("{val}");
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {received}")

    // Example multivalue channels
    // let handle = thread::spawn(move || {
    //     let vals = vec![
    // String::from("hi"),
    // String::from("from"),
    // String::from("the"),
    // String::from("thread"),
    //     ];
    //     for val in vals {
    //         tx.send(val).unwrap();
    //     }
    // });

    // for received in rx {
    //     println!("Got: {received}");
    // }

    // multiproducer by colining the transfmitter
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got {received}!");
    }

    // handle.join().unwrap();
}
