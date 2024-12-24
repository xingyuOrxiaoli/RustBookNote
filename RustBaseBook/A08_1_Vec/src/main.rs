
fn  main() {
    println!(" 第八章 集合 ");
    println!("\t vector 允许我们一个挨着一个地储存一系列数量可变的值");
    println!("\t 字符串（string）是字符的集合");
    println!("\t 哈希 map（hash map）允许我们将值与一个特定的键（key）相关联");
    println!("========================== Vec<T> ==================================");
    let mut v1 :Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    
    v1.push(4);
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);
    
    println!(" v1 = {:?} , v2 = {v2:?}", v1);
    
    // 遍历
    let v2_second :&i32 = &v2[1];
    println!("v2_second = {:?}", v2_second);
    let v2_third :Option<&i32> = v2.get(2);
    match v2_third {
        Some(third) => println!("v2_second = {:?}", third),
        None => println!("v2_second none")
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;  // ，在使用 += 运算符之前必须使用解引用运算符（* ）获取 i 中的值
    }
}
