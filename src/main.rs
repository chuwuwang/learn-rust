extern crate connector;

mod book_pal;
mod communicator;

fn main() {
    // communicator::client::connect();
    // connector::client::connect();

    // guess();
    // book_pal::strings_test::strings_test();
    // book_pal::map_test::map_test();
    // book_pal::scope_test::scope_test();
    book_pal::checked::checked::op(211.0, 10.0).unwrap();

}