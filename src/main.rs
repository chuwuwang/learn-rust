extern crate connector;

mod book_pal;
mod communicator;

fn main() {

    // guess();

    let rect1 = book_pal::rectangles::Rectangle { width: 30, height: 50 };
    // println!("rect1 is {:?}", rect1);
    // rect1.area();

    communicator::client::connect();
    connector::client::connect();

    book_pal::strings_test::strings_test();
    book_pal::map_test::map_test();

}