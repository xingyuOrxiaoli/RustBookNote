
fn main() {
    println!("第四章 所有权  引用与借用");
    
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}."); // 这里s1的所有权还在并且没有被drop 因此这里s1还能使用
    {
        let r2 = &mut s1;
        println!("{}", r2);
    } // // r2 在这里离开了作用域，所以我们完全可以创建一个新的引用
    let r1 = &mut s1;
    println!("{r1}"); // 只能有一个可变借用 并且不能拥有其他任何借用
}
fn calculate_length(s: &String) -> usize { // s 是 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
// 所以什么也不会发生