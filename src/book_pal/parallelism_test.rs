async fn hello() {
    println!("Hello, world!");
}

async fn run() {
    for i in 0..10 {
        println!("{}", i);
    }
}

pub async fn parallelism_test() {
    tokio::spawn( run() );
    hello().await;
}