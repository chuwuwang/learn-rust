use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub fn channel_test() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(i).unwrap();
            // 发送是一个非阻塞（non-blocking）操作, 线程将在发送完消息后. 会立即继续进行
            println!("Sent: {}", i);
        } );
    }
    let mut ids = Vec::with_capacity(3usize);
    for _ in 0..3 {
        let id = rx.recv().unwrap();
        ids.push(id);
    }
    println!("Received ids: {:?}", ids);
}