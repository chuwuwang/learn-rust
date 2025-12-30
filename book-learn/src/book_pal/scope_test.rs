pub fn scope_test() {
    let s = String::from("Hello World");

    take_ownership(s);

    let x = 5;

    makes_ownership(x);

    println!("x: {}", x);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_ownership(some_string: i32) {
    println!("{}", some_string);
}