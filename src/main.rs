#![allow(dead_code)]

extern crate connector;

mod book_pal;
mod communicator;

use crate::book_pal::file_test;
use crate::book_pal::process_test;

use crate::book_pal::arc_test;
use crate::book_pal::channel_test;
use crate::book_pal::condvar_test;
use crate::book_pal::{ rayon_test };

fn main() {
    // communicator::client::connect();
    // connector::client::connect();

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
    rayon_test::rayon_test2()
}