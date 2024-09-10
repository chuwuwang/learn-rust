fn vector_test() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third = &v[2];
    let third = v.get(2);

    for i in &v { // 不可变引用
        println!("{}", i);
    }

    for i in &mut v { // 可变引用
        *i += 50;   // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符( * )获取 i 中的值。
    }

}