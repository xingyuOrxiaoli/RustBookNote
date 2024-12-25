

fn print_and_returns_10(a:i32)-> i32{
    println!("I got the value {}", a);
    a+10
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn this_test_will_pass(){
        let value = print_and_returns_10(10);
        assert_eq!(value,20);
    }
    
    #[test]
    fn this_test_will_fail(){
        let value = print_and_returns_10(10);
        assert_eq!(value,20);
    }
    
    #[test]
    #[ignore]
    fn expensive_test(){
        // 可能需要运行 一个小时的代码 使用 #[ignore] 可以忽略
        assert!(true);
    }
    
}
