
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Debug)]
struct AlwaysEqual;
fn main() {
    println!("第五章 元组结构体");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black = {:?}  origin={:?}",black,origin);
    println!("第五章 没有任何字段的类单元结构体");
    let subject = AlwaysEqual;
    println!("subject = {:?}" , subject);


}
