
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Rust 程序设计语言 书的第二章 写一个猜数字的游戏!");

    // 生成一个随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("生成的随机数为 ： {}",secret_number);
    //cargo doc --open1

    loop {
        println!("请输入你猜的数字(范围为 1- 100):");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("错的输入");

        println!("你猜的数字为 ： {}", guess);

        // 字符串转换数字
        // let guess: u32 = guess.trim().parse().expect("请输入数字，转换失败");
        // 处理无效的输入
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("你猜小了"),
            Ordering::Equal => {
                println!("你猜对了") ;
                break;
            },
            Ordering::Greater => println!("你猜大了")
        }
    }



}
