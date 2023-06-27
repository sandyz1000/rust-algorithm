#![allow(unused)]

use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};
use std::ptr;


#[derive(Debug)]
struct Config {
    db_connection_str: String,
}

fn get_config() -> &'static Mutex<Config> {
    static mut CONF: MaybeUninit<Mutex<Config>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        CONF.as_mut_ptr().write(Mutex::new(Config {
            db_connection_str: "test config".to_string(),
        }));
    });

    unsafe { &*CONF.as_ptr() }
}

fn main() {
    let f1 = get_config();
    println!("{:?}", f1);
    // modify
    {
        let mut conf = f1.lock().unwrap();
        conf.db_connection_str = "hello".to_string();
    }

    let f2 = get_config();
    println!("{:?}", f2);
    let conf2 = f2.lock().unwrap();

    assert_eq!(conf2.db_connection_str, "hello".to_string())
}



/// The Singleton struct has a private constructor, so it can only be instantiated from within the struct itself. 
/// The get_instance method is used to retrieve the singleton instance and ensures that only one instance of the 
/// Singleton struct is ever created. The static mut variable SINGLETON_INSTANCE is used to store the singleton 
/// instance, and Once is used to ensure that the singleton is only instantiated once, even if multiple threads 
/// attempt to access it simultaneously.
///
/// Note that the use of unsafe is necessary here because we are dealing with raw pointers and need to ensure that 
/// the lifetime of the Singleton instance is properly managed.
struct Singleton {
    // Fields go here
}

// The static variable which stores the singleton instance
static mut SINGLETON_INSTANCE: *const Singleton = ptr::null();
static ONCE: Once = Once::new();

impl Singleton {
    // Private constructor to prevent external instantiation
    fn new() -> Singleton {
        Singleton {
            // Initialize fields here
        }
    }

    // Method to get the singleton instance
    fn get_instance() -> &'static Singleton {
        unsafe {
            ONCE.call_once(|| {
                let singleton = Singleton::new();
                SINGLETON_INSTANCE = Box::into_raw(Box::new(singleton));
            });
            &*SINGLETON_INSTANCE
        }
    }
}

#[test]
fn test() {
    let singleton1 = Singleton::get_instance();
    let singleton2 = Singleton::get_instance();
    assert_eq!(singleton1 as *const Singleton, singleton2 as *const Singleton);
}
