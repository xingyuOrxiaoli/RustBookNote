

fn main() {
    println!("常见编程概念");

    println!("---------- 变量和可变性 ----------");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("---------- 常量 ----------");
    // 不允许对常量使用mut
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The const value is {}",THREE_HOURS_IN_SECONDS);

    println!("---------- 隐藏 ----------");
    let x = 5;
    let x = x + 2 ;
    {
        let x = x * 2;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);

    println!("---------- 数据类型 ----------");
    // Rust是静态类型语言，也就说在编译的时候必须知道所有变量的类型
    let guess : u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);
    println!("标量类型，代表一个单独的值。Rust有四种基本的标量类型：整型、浮点型、布尔类型和字符类型");
    println!("\t 整型 i8 i16 i32 i64 i128 isize");
    println!("\t 整型 u8 u16 u32 u64 u128 usize");
    println!("\t 整型  8  16  32  64  128 arch");
    println!("\t 整型字面值：二进制0b1111_0000 八进制0o77 十进制98_22 十六进制0xff 单字节字符(仅限于u8)b'A'");

    let x = 2.0 ; // f64
    let y: f32 = 3.0 ; // f32
    let flag_x = true;
    let flag_y = false;

    let str_x = 'z';
    let str_y :char = 'ℤ';
    let str_z = ' ';
    println!("x = {x} , y = {y}, flag_x = {flag_x} flag_y = {flag_y} str_x={str_x} str_y={str_y} str_z={str_z}");

    println!("---------- 复合类型 ： 元组（tuple）和数组（array） ----------");
    println!("元组类型:");
    let tup_x:(i32,f64,u8) = (1, 2.0, 3);
    let tup_y =(1,2.0,3);
    let (x,y,z) = tup_x;
    println!("\t The value of x is: {}", x);
    println!("\t The value of y is: {}", y);
    println!("\t The value of z is: {}", z);
    println!("\t tup_y.0 = {}",tup_y.0);
    println!("\t tup_y.1 = {}",tup_y.1);
    println!("\t tup_y.2 = {}",tup_y.2);
    println!("数组类型:");
    let array_x = [1,2,3,4,5];
    let array_y =["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let array_z:[i32;6]=[1,2,3,4,5,6];
    let array_x_x = [3;5];
    println!("\t array_x = {:?} array_y = {:?} array_z = {:?} array_x_x = {:?}",array_x,array_y,array_z,array_x_x);








}
