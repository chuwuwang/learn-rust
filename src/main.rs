extern crate connector;

mod book_pal;
mod communicator;

fn main() {

    // guess();

    communicator::client::connect();
    connector::client::connect();

    // book_pal::strings_test::strings_test();

    // book_pal::map_test::map_test();

    book_pal::scope_test::scope_test();

}