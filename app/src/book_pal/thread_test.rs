use std::thread;

pub fn scoped_threads() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;
    thread::scope( |s| {
        s.spawn( || {
            println!("Scope thread 1: {:?}", &a);
        } );
        s.spawn( || {
            x += a[0] + a[2];
        } );
    } );
    a.push(4);
    assert_eq!(x, 4);
}