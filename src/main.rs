use crate::book_pal::guessing_game::guess;

mod book_pal;

fn main() {

    // guess();

    let rect1 = book_pal::rectangles::Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1);
    // rect1.area();

}