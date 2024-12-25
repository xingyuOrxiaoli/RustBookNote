
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle{
    width: usize,
    height: usize,
}
impl Rectangle {
    fn can_hold(&self,other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn add_two(a : i32) -> i32{
    a + 2
}
pub fn greeting(name : &str) -> String {
    format!("Hello {}!", name)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    // 检查panic
    #[should_panic(expected = "Make this test fail")]
    fn another(){
        panic!("Make this test fail");
    }
    
    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{width: 8, height: 3};
        let smaller = Rectangle{width: 2, height: 2};
        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle{width: 8, height: 7};
        let smaller = Rectangle{width: 5, height: 1};
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn it_adds_two(){
        assert_eq!(4, add(2, 2));
    }
    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert_ne!(1,1);
        assert!(result.contains("Carol"));
    }
    
    // Result<T,E>用于测试
    // Result 不能使用 #[should_panic] 注解
    #[test]
    fn it_works_result() ->Result<(), String>{
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        }else { 
            Err(String::from("This test fails"))
        }
    }
}