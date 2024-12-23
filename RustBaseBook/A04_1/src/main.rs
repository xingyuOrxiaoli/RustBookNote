

fn main() {
    println!("第四章 认识所有权");
    println!("\t 1. Rust 中的每一个值都有一个 所有者（owner）。");
    println!("\t 2. 值在任一时刻有且只有一个所有者。");
    println!("\t 3. 当所有者（变量）离开作用域，这个值将被丢弃。");
    
    println!(" ---------- 变量域 --------- ");
    { // s 在这里无效，它尚未声明
        let s = "hello"; // 从此处起，s 是有效的
        println!("The s value is  {}",s) ; // 使用 s
    } // 此作用域已结束，s 不再有效

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`
    
    println!("移动  对于存储数据在栈中的数据 x还可以使用，因为他是直接拷贝");
    let x = String::from("hello");
    let y = x;
    println!("y = {}",y);
    
    let str_x = String::from("hello");
    let str_y = str_x.clone();
    println!("str_x = {} str_y={}",str_x,str_y);
    
    
    
    

}
