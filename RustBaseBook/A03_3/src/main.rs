
fn main() {
    println!("常见编程概念  控制流");
    println!(" if 表达式");
    let mut number = 3;
    if number > 4{
        println!(" number 大于 4");
    }else if number < 4{ 
        println!(" number 小于 4")
    }else { 
        println!(" number 等于 4")
    }
    println!("=================================");
    let condition = true;
    let result = if condition {4} else {5};
    println!("The result is {}", result);


    println!(" loop 循环重复 表达式");
    let result = loop {
        if number == 1 {
            break number * 10; 
        }
        number -= 1;
        println!(" loop 重复循环 表达式 执行 ");
    };
    println!("result: {}", result);
    println!("================ 循环标签 =================");
    let mut count = 2;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    println!("================ while =================");
    let mut number_while = 3;
    while number_while != 0 {
        println!("The number_while is {}", number_while);
        number_while -= 1;
    }
    println!("================ for =================");
    let array_for = [10, 20, 30, 40, 50];
    for element in array_for {
        println!(" for in : the value is: {element}");
    }
    for number in (1..4).rev() {
        println!("for in (start...end).rev() : {}!", number);
    }
    
    


}
