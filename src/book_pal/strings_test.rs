pub fn strings_test() {
    let data = "hello world";
    let s = data.to_string();

    let s = String::from("hello world");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {}", s2);
    println!("s1 is {}", s1);

    let s3 = s1 + &s2;
    println!("s3 is {}", s3);
    println!("s2 is {}", s2);

    for i in "hello world".chars() {
        println!("{}", i);
    }

}