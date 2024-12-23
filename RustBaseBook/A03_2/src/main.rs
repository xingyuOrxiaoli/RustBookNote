


fn main() {
    
    println!("常见编程概念  函数");
    
    function_1();
    function_2(12);
    function_3(21,'c');
    function_4();
    function_5(323);
    
    println!("============= 表达式 ===============");
    let y = {
        let x = 4;
        x +1
    };
    println!("y = {}", y);
    
    
    
}
fn function_1(){
    println!(" function_1 无参 无返回值的函数");
}
fn function_2(x:i32){
    println!(" function_2 有单个参x={} 无返回值的函数",x);
}
fn function_3(x:i32 , y:char){
    println!(" function_3 有多个参x={},y={} 无返回值的函数",x,y);
}
fn function_4() -> i32{
    println!(" function_4 无参 有返回值的函数");
    6
}
fn function_5(x:i32) -> i32{
    println!(" function_2 有单个参x={} 有返回值的函数",x);
    x +  1
}




