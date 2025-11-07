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

pub fn channel_safe() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("Hello from thread").unwrap();
        // 发送后tx所有权转移, 主线程无法再使用
    } );
    // 主线程接收消息, 阻塞等待, 接收到 "Hello from thread"
    let msg = rx.recv().unwrap();
    println!("{}", msg);
}