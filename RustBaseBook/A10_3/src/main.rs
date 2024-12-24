
// 生命周期注解语法 
// 生命周期注解并不改变任何引用的生命周期的长短。相反它们描述了多个引用生命周期相互的关系，而不影响其生命周期。
// 生命周期参数名称必须以撇号（' ）开头，其名称通常全是小写，类似于泛型其名称非常短。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体定义中的生命周期注解
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    println!("第十章  生命周期部分 !");


    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    println!("  -------- 结构体定义生命周期注解 ----------");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    {
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("i = {i:?}");
    }
    println!("novel = {novel}");

    println!("  -------- 静态生命周期 ----------");
    let s : &'static str = "ni hao shi jie";
    
    
    println!("s = {s}");
}


// 泛型和生命周期集合
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}