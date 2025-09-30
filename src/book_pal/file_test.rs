use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn file_test() {
    let path = Path::new("hello.txt");
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