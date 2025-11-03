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

#[tokio::main]
pub async fn async_tokio_test() {
    hi().await;
}