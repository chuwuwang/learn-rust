use std::collections::HashMap;

pub fn map_test() {
    let mut map = HashMap::new();
    map.insert("key1", 10);

    let teams = vec!["Blue", "Red"];
    let vec1 = vec![10, 50];
    let kx: HashMap<_, _> = teams.iter().zip(vec1.iter()).collect();

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
        print!("No value found");
    }

    panic!("这是一个 panic 示例");

}