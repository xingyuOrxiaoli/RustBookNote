
use std::collections::HashMap;

fn main() {
    println!("第八章 集合 HashMap  默认使用SipHash的哈希函数 它可以抵御设计哈希表的拒绝服务Dos攻击 ");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let team_name = String::from("Blue");
    // copied() 方法获取的是一个 Option<i32>  不是 Option<&i32>
    // unwrap_or(0) 在scores中没有改键所对应的项时将其设置为零
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("team score is {:?}", score);
    println!( " --------------- HashMap集合遍历 ----------------");
    for (key,value) in &scores{
        println!("{},{}", key,value);  // 打印顺序是任意顺序，不一定是输入顺序
    }

    println!( " --------------- String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者 ----------------");
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！
    println!("{map:?}");
    println!( " --------------- 只在键没有对应值时插入键值对 ----------------");
    map.insert(field_name, field_value);
    let field_name = String::from("Favorite color");
    let field_value = String::from("Red");
    // 这里添加的key 和value 不是拷贝 因此这里的所有权将会移动
    // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用
    map.entry(field_name).or_insert(field_value);
    println!("{map:?}");
    println!( " --------------- 根据旧值更新一个值 ----------------");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
    

}
