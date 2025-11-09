#![allow(dead_code)]

extern crate connector;

mod book_pal;
mod communicator;

use std::future;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*};
use tokio::runtime;
use tokio::runtime::Runtime;

use crate::book_pal::{file_test, reqwest_test, tokio_test};
use crate::book_pal::process_test;
use crate::book_pal::arc_test;
use crate::book_pal::channel_test;
use crate::book_pal::condvar_test;
use crate::book_pal::{ rayon_test };

fn main() {
    let timme = fmt::time::LocalTime::rfc_3339();
    let subscriber = fmt::Subscriber::builder()
        .with_timer(timme) // 本地时间格式
        .with_level(true) // 显示日志级别
        .with_span_events(fmt::format::FmtSpan::ACTIVE) // 显示跨度激活/结束
        .finish();
    // 设置全局订阅者
    tracing::subscriber::set_global_default(subscriber).expect("初始化订阅者失败");
    // 后续的追踪操作会通过该订阅者处理
    info!("程序启动");

    // communicator::client::connect();
    // connector::client::connect();
    let result = Runtime::new().unwrap().block_on( connector::test_server() );
    println!("{:?}", result);

    // guess();
    // book_pal::strings_test::strings_test();
    // book_pal::map_test::map_test();
    // book_pal::scope_test::scope_test();
    // book_pal::checked::checked::op(211.0, 10.0).unwrap();

    // file_test::file_test();
    // file_test::file_create_test();
    // process_test::process_test();

    // arc_test::arc_test()
    // arc_test::test_shared_counter()
    // condvar_test::condvar_test()
    // channel_test::channel_test()
    // channel_test::channel_safe()
    // rayon_test::rayon_test()
    // rayon_test::rayon_test2()

    // let future = reqwest_test::request_test();
    // tokio_test::async_tokio_test();
}