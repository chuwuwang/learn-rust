use crate::book_pal::{reqwest_test, tokio_test, anyhow_test, user};
use tokio::runtime::Runtime;

pub fn test_book_pal() {
    let result = Runtime::new().unwrap().block_on(connector::test_server());
    println!("{:?}", result);

    user::json_test();

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
    tokio_test::async_tokio_test();

    let future = anyhow_test::anyhow_test();
}