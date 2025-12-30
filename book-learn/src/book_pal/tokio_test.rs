use std::{future, thread};
use std::thread::Builder;
use tokio::runtime;
use futures::future::{Abortable, AbortHandle};
use tokio::time::{sleep, Duration};

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

#[tokio::main]
pub async fn tokio_cancel() {
    // 创建 abort handle
    let (abort_handle, abort_registration) = AbortHandle::new_pair();

    // 构建一个可取消的 Future
    let future = Abortable::new(async {
        for i in 0..10 {
            println!("Working on task {i}");
            let duration = Duration::from_secs(1);
            sleep(duration).await;
        }
        "Task completed"
    }, abort_registration);

    // 启动任务
    let task = tokio::spawn(future);

    // 超时模拟
    let duration = Duration::from_secs(5);
    tokio::time::sleep(duration).await;

    // 发出取消信号
    abort_handle.abort();

    match task.await {
        Ok( Ok(result) ) => println!("Task Complete - {}", result),
        Ok( Err(_) ) => println!("Task was cancelled"),
        Err(e) => println!("Task panic: {}", e),
    }
}