use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn file_test() {
    let path = Path::new("../../hello.txt");
    let display = path.display();
    println!("display: {:?}", display);

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", display, why),
    };

    let mut str = String::new();
    file.read_to_string(&mut str).expect("read failed");
    println!("file content: {}", str);
}

static LOREM_IPSUM: &'static str ="Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmodtempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodoconsequat. Duis aute irure dolor in reprehenderit in voluptate velit essecillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat nonproident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub fn file_create_test() {
    let path = Path::new("../../rust.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {:?}", display, why),
        Ok(file) => file,
    };

    let bytes = LOREM_IPSUM.as_bytes();
    match file.write_all(bytes) {
        Ok(_) => println!("successfully wrote to {}", display),
        Err(why) => panic!("couldn't write to {}: {:?}", display, why),
    }
}