mod int_mut;

use crate::List::{Cons, Nil};
use std::{fmt::Debug, ops::Deref, rc::Rc};
fn main() {
    let b = Box::new(5);
    println!("{b}");

    // Cons list made recursive using boxes.
    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    println!("{list:?}");

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Implementing own smart pointer.
    let z = MyBox::new(x);
    let name = MyBox::new(String::from("Jahir"));

    assert_eq!(5, *z);
    hello(&name);

    // Reference Counting
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("{b:?} and {c:?}");
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

/// Defining MyBox Smart Pointer
///
#[derive(Debug)]
struct MyBox<T: std::fmt::Debug>(T);

impl<T: std::fmt::Debug> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: std::fmt::Debug> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {name}");
}

impl<T: std::fmt::Debug> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping {:?}", self.0)
    }
}
