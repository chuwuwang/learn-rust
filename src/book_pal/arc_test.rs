use std::sync::Arc;
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