// https://betterprogramming.pub/rust-singleton-application-db2df027afa1

#![allow(unused)]
use std::rc::{Weak, Rc};

struct Singleton {
    instance: Weak<Self>,
    count: u8
}

impl Singleton {
    pub fn get_instance() -> Rc<Singleton> {
        Rc::new_cyclic(|me| Singleton { instance: me.clone(), count: 0 })
    }
}

fn instance_and_use_singleton() {
    let mut singleton = Singleton::get_instance();
    let mut count = singleton.count;
    println!("singleton: {}", count);
    count += 1;
    println!("singleton: {}", count);
}

#[test]
fn main() {
    instance_and_use_singleton();
    instance_and_use_singleton();
}