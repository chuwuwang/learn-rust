#![allow(dead_code)]
mod book_pal;
mod core_network;
mod feature_hardware;

use crate::book_pal::test;

use tracing::info;
use tracing_subscriber::fmt;

fn main() {
    init_logging();

    core_network::main::main();

    feature_hardware::main::main();

    test::test_book_pal();
}

fn init_logging() {
    let timme = fmt::time::LocalTime::rfc_3339();
    let subscriber = fmt::Subscriber::builder()
        .with_timer(timme) // 本地时间格式
        .with_level(true) // 显示日志级别
        .with_span_events(fmt::format::FmtSpan::ACTIVE) // 显示跨度激活/结束
        .finish();
    // 设置全局订阅者
    tracing::subscriber::set_global_default(subscriber).expect("init tracing subscriber failed");
    // 后续的追踪操作会通过该订阅者处理
    info!("App started");
}