use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    // 自己定义了一个闭包，并将其作为变量存储下来
    let expensive_closure = |num : u32| {
        println!(" calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups", expensive_closure(intensity));
        println!("Next, do {} situps", expensive_closure(intensity));
    }else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }else {
            println!("Today, run for {} minutes!",expensive_closure(intensity));
        }
    }
}

#[derive(Debug,PartialEq,Copy,Clone)]
enum ShirtColor{
    Red,
    Blue,
}

struct Inventory{
    shirt:Vec<ShirtColor>,
}

impl Inventory{
    fn giveaway(&self,user_preference:Option<ShirtColor>) -> ShirtColor{
        // 这里使用了闭包
        user_preference.unwrap_or_else(||self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor{
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirt{
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        }else {
            ShirtColor::Blue
        }

    }
}


// 测试slice上的标准库方法sort_by_key 准备
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}


fn main() {
    println!(" 迭代器与   闭包!");
    println!("\t 闭包，一个可以存在变量里的类似函数的结构");
    println!("\t 迭代器，一种处理元素序列的方式");

    let store = Inventory{
        shirt: vec![ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("user_pref1 {:?}  \t giveaway1 {:?}",user_pref1, giveaway1);

    let user_pref2  = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("user_pref2 {:?}  \t giveaway2 {:?}",user_pref2, giveaway2);

    println!(" ================================================================= ");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
    println!(" ================================================================= ");
    // 不同写法的对比
    fn add_one_v1   (x:u32)   ->  u32 { x + 1}
    let add_one_v2 =    |x :u32| {x +1};
    let add_one_v3    = |x| {x +1};
    let add_one_v4    = | x | x+1;
    let res3 = add_one_v3(44);
    let res4 = add_one_v4(1) ;
    println!("res3 = {}",res3);
    println!("res4 = {}",res4);
    println!(" ============================= 不可变借用 ==================================== ");
    let mut list = vec![1, 2, 3];
    println!("{:?}",list);
    let only_borrows = || println!(" From only borrows {list:?}");
    println!("only borrows before  => {list:?}");
    only_borrows();
    println!("only borrows after   => {list:?}" );
    println!(" --- borrows_mutably");
    let mut borrows_mutably = || list.push(4);
    borrows_mutably();
    println!(" --- borrows_mutably  => {list:?}" );

    println!(" ==== move 关键字 ===");
    // 使用move关键字 强制获取list的所有权
    thread::spawn(move || println!("From thread  {list:?}"))
        .join()
        .unwrap();
    println!(" ===== 将被捕获的值移除闭包和 fn trait =====");
    println!("\t 1.FnOnce 适用于只能被调用一次的闭包     => unwrap_or_else()");
    println!("\t 2.FnMut  适用于不会将捕获的值移出闭包体，但可能会修改捕获值的闭包。 => sort_by_key");
    println!("\t 3.Fn 适用于既不将捕获的值移出闭包体，也不修改捕获值的闭包，同时也包括不从环境中捕获任何值的闭包。");
    
    let mut list = [
        Rectangle{width:10 , height:1},
        Rectangle{width:3 , height:5},
        Rectangle{width:7 , height:12},
    ];
    list.sort_by_key(|r| r.width);
    println!("{:?}",list);

    println!(" ================== 使用迭代器处理元素序列 =======================");
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter{
        println!("v1_iter = {:?}",val);
    }
    
    println!(" ================== 产生其他迭代器的方法 =======================");
    // 产生其他迭代器的方法 
    // 不会消费当前迭代器，而是通过改变原始迭代器的某些方面来生成不同的迭代器
    let v1 :Vec<i32> = vec![1,2,3];
    let v2 :Vec<_> = v1.iter().map(|x| x + 1 ).collect();
    println!("v1 = {v1:?}");
    println!("v2 = {v2:?}");
    assert_eq!(v2 , vec![2,3,4]);
    

}
