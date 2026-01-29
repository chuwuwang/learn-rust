pub fn swap_demo() {
    let mut x = String::from("Hello");
    let mut y = String::from("World");

    std::mem::swap(&mut x, &mut y);
    tracing::info!("x: {}, y: {}", x, y);
}

fn fast_remove<T>(vec: &mut Vec<T>, index: usize) -> T {
    let last_index = vec.len() - 1;
    vec.swap(index, last_index);
    vec.pop().unwrap()
}
