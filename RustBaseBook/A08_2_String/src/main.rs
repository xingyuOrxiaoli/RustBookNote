
fn main() {
    println!("第八章 集合 String  UTF-8编码的文本");

    println!("Rust 的核心语言中只有一种字符串类型  字符串slice &str");
    let s1 = String::new();
    let str1 = "ni hao shi jie";
    let s2 = str1.to_string();
    // 该方法也可直接用于字符串字面值：
    let s3 = "ni hao shi jie".to_string();
    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
    println!(" ============================================== ");
    
    let mut s4 = String::from("foo");
    s4.push_str("bar");
    let s5 = " add  s5";
    s4.push_str(s5);
    println!("s4: {}", s4);
    let s6 = " use + add";
    // 使用规则 String  + &str + &str + ....
    // 底层是调用 fn add(&self ,s &str) 这个函数
    s4 = s4 + s6;  // 第一个的所有权会被移动，后面是slice &str 不会被
    // &String 可以被强转 &str   这里底层调用参数为 &str类型  但是Rust使用了一个被称为Deref强制转换的技术
    println!("s4: {}", s4);


    println!(" =====================   format!宏  ========================= ");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    
    println!("不支持索引 s[index]的用法，因为String 底层是一个Vec<u8>的封装 ");
    println!("字节、标量值和字形簇")
    // 最后一个 Rust 不允许使用索引获取 String 字符的原因是，索引操作预期总是需要常数时间（O(1)）。
    // 但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符
    
}
