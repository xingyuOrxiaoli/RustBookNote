use std::error::Error;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};
//  传播错误
fn read_username_from_file1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 传播错误的简写   ?
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
fn main() -> Result<(),Box<dyn Error>>{
    println!("第九章 错误处理 panic不可恢复的错误  -- 传播错误");
    //的 match 表达式与 ? 运算符所做的有一点不同：? 运算符所使用的错误值被传递
    // 给了 from 函数，它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类
    // 型。当 ? 运算符调用 from 函数时，收到的错误类型被转换为由当前函数返回类型所指定的
    // 错误类型。这在当函数返回单个错误类型来代表所有可能失败的方式时很有用，即使其可能会
    // 因很多种原因失败。
    let greeting_file = File::open("hello.txt")?;
         
    Ok(())
}