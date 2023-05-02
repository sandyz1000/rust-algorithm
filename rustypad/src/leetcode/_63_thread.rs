//
#![allow(unused)]
use std::rc::Rc;
use std::thread;
use std::thread::JoinHandle;


struct Solution;

impl Solution {
    fn thread_exe() {
        let numbers = Vec::from_iter(0..=1000);

        let t: JoinHandle<usize> = thread::spawn(move || {
            let len = numbers.len();
            let sum = numbers.iter().sum::<usize>();
            sum / len
        });

        let average = t.join().unwrap();

    }

    fn thread_tuts() {
        let numbers = Vec::from_iter(0..=1000);
        
        thread::spawn(move || {
            for n in numbers {
                println!("{n}");
            }
        }).join().unwrap();
    }

    fn thread_scope() {
        let numbers = vec![1, 2, 3];
        // println!("{}", numbers[3]);
        thread::scope(|s| {
            s.spawn(|| {
                println!("length: {}", numbers.len());
            });
            s.spawn(|| {
                for n in &numbers {
                    println!("{n}");
                }
            });
        });

    }
    fn reference_counter() {
        let a = Rc::new([1, 2, 3]);
        let b = a.clone();

        assert_eq!(a.as_ptr(), b.as_ptr()); // Same allocation!
    }
}

