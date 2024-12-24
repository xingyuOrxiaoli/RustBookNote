
use std::fs::File;
use std::io::ErrorKind;

fn main() {

    println!("第九章 错误处理 panic不可恢复的错误");
    
    let greeting_file_result = File::open("Result.txt");
    let greeting_file1  = match greeting_file_result { 
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Result.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the result file: {:?}", error),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };

    println!("使用闭包实现 简化match");
    let greeting_file2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
    // panic!("crash and burn");

}
