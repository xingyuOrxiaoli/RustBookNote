
#[derive(Debug)]
struct User{
    username: String,
    password: String,
    age:u32,
}
impl User {
    fn show(&self){
        println!("你的账号为:{},密码为：{}",self.username,self.password);
    }
    fn new(age :u32)->User{
        Self{
            username:String::from("username"),
            password:String::from("password"),
            age:age,
        }
    }
}
fn main() {
    println!("第五章  结构体");
    
    let mut user1 = User{
        username:String::from("username"),
        password:String::from("password"),
        age:22
    };
    user1.username = String::from("username_update");
    println!("user1 = {:?}",user1);
    
    let  username:String = String::from("username2");
    let  password:String = String::from("password2");
    let  age :u32 = 22;
    let  user2 = User{
        username,
        password,
        age,
    };
    println!("user2 = {:?}",user2);
    println!("{}",user2.password);
    println!("结构体对象更新语法");
    let user3 = User{
        username:String::from("username3"),
        // password:String::from("password"),
        // 如果更新的内容设计非栈数据，user2将不能在之后使用，因为   其user2的所有权被移动
        age:12,
        ..user2  
        
    };
    // println!("user2 = {:?}",user2);
    println!("user3 = {:?}",user3);
    
    println!(" ------------------ dbg!的使用 ------------------");
    let age  = 11;
    let user4 = User{
        username:String::from("username4"),
        password:String::from("password4"),
        age:dbg!(2 * age),
    };
    dbg!(user4);
    println!(" ------------------ 结构体定义方法 ------------------");
    let user5 = User{
        username:String::from("username5"),
        password:String::from("password"),
        age:22,
    };
    user5.show();
    println!(" ------------------ 不含&self 参数的关联函数------------------");
    let user6 = User::new(22);
    println!("user6 = {:#?}",user6);
    
    
}
