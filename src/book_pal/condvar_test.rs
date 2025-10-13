use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub fn condvar_test() {
    let pair = Arc::new((Mutex::new(false), Condvar::new() ) );
    let pair2 = Arc::clone(&pair);
    thread::spawn(move || {
            println!("thread started");
            let (lock, cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            println!("thread locked");
            *started = true;
            cvar.notify_one();
            println!("thread finished");
        }
    );
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while ! *started {
        println!("while started");
        started = cvar.wait(started).unwrap();
        println!("while finished");
    }
    println!("done");
}