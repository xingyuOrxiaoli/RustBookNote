
#[derive(Debug)]
struct Point<T,U>{
    x:T,
    y:T,
    z:U,
}

impl <T,U> Point<T,U>{
    fn new(x:T,y:T,z:U)->Point<T,U>{
        Point{x,y,z}
    }
}
// 定义方法时也可以为泛型指定限制（constraint）
impl Point<f32,f32>{
    fn distance_from_origin(&self)->f32{
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
}
//泛型代码的性能
// 在阅读本部分内容的同时，你可能会好奇使用泛型类型参数是否会有运行时消耗。好消息是泛
// 型并不会使程序比具体类型运行得慢。
// Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个
// 通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程

fn main() {
    println!("泛型、Trait 和生命周期");
    
    let wont_work1 = Point{x:1,y:1,z:10};
    let wont_work2 = Point{x:2.0,y:2.0,z:10};
    println!("wont_work1 = {wont_work1:?},wont_work2 = {wont_work2:?}");
    
}
