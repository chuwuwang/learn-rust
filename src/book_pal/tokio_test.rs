use std::thread;
use std::thread::Builder;
use std::time::Duration;
use tokio::runtime;

async fn hi() {
    println!("Hello hi");
}

pub fn tokio_test() {
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on( hi() );
}

async fn hello(task: u64, time: u64) {
    println!("Task {task} started.");
    let dur = Duration::from_millis(time);
    // thread::sleep(dur);
    tokio::time::sleep(dur).await;
    println!("Task {task} finished.");
}

#[tokio::main]
pub async fn async_tokio_test() {
    // hi().await;

    tokio::join!(
        hello(1, 1000),
        hello(2, 500),
        hello(3, 2000),
        hello(4, 1000),
        hello(5, 500),
        hello(6, 2000),
    );

    // let rt = runtime::Builder::new_multi_thread()
    //     .worker_threads(4)
    //     .enable_all()
    //     .build()
    //     .unwrap();
    // rt.block_on(async {
    //     tokio::join!(
    //         hello(1, 1000),
    //         hello(2, 500),
    //         hello(3, 2000),
    //         hello(4, 1000),
    //         hello(5, 500),
    //         hello(6, 2000),
    //     )
    // } );
}