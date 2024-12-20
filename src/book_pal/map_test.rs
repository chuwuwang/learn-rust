use std::collections::HashMap;

pub fn map_test() {
    let mut map = HashMap::new();
    map.insert("key1", 10);

    let teams = vec!["Blue", "Red"];
    let vec1 = vec![10, 50];
    let mut kx: HashMap<_, _> = teams.iter().zip(vec1.iter()).collect();

    let option = map.get("key1");
    match option {
        Some(value) => println!("Value: {}", value),
        None => println!("No value found"),
    }
    let xx = option.unwrap();
    print!("Value: {}", xx);
    let bo = option.is_none();
    let bo = option.is_some();
    if let Some(value) = option {
        println!("Value: {}", value);
    } else {
        println!("No value found");
    }

    // panic!("这是一个 panic 示例");

    let mut source = HashMap::new();
    source.insert("Yellow", 30);
    source.entry("Blue").or_insert(50);
    println!("{:?}", source);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    panic!("crash and burn");

    println!("end of main");
}