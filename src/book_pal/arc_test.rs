#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use std::thread;

pub fn arc_test() {
    let string = Arc::new("hello world");
    for _ in 0..10 {
        let string = Arc::clone(&string);
        thread::spawn(move || {
            println!("{}", string);
        } );
    }
}

struct SharedCounter {
    data: Arc< Mutex<i32> >,
}

impl SharedCounter {

    fn new(initial: i32) -> Self {
        SharedCounter {
            data: Arc::new( Mutex::new(initial) ),
        }
    }

    fn increment(&self) {
        let mut count = self.data.lock().unwrap();
        *count += 1;
    }

    fn get(&self) -> i32 {
        let count = self.data.lock().unwrap();
        *count
    }

}

impl Clone for SharedCounter {

    fn clone(&self) -> Self {
        SharedCounter {
            data: self.data.clone(), // 内部使用 Arc::clone
        }
    }

}

trait Handle: Clone {

    fn handle(&self) -> Self {
        Clone::clone(self)
    }

}

impl Handle for SharedCounter {}

pub fn test_shared_counter() {

    let counter = SharedCounter::new(0);
    let counter_handle = counter.handle();

    thread::spawn(move || {
        for i in 0..10 {
            counter_handle.increment();
        }
    } );
    thread::sleep( std::time::Duration::from_millis(100) );
    let value = counter.get();
    println!("counter value is {}", value);
}