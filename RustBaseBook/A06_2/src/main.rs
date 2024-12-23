


#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin:Coin)->u8{
    match coin{
        Coin::Penny=>{
            println!("Lucky penny!");
            1
        },
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quarter(state)=>{
            println!("State quarter from {state:?}!");
            25
        },
    }
}
fn main() {
    println!("第六章 match 控制流结构");
    value_in_cents(Coin::Quarter(UsState::Alaska));
    println!(" ========================= ");
    fn plus_one(x:Option<i32>) -> Option<i32>{
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }
    let five = Some(5);
    let six  = plus_one(five);
    let none = plus_one(None);
    println!("six = {:?}",six);
    println!("none = {:?}",none);
    println!(" ========================= ");
    fn add_fancy_hat(){}
    fn remove_fancy_hat(){}
    fn move_player(num_spaces :u8){
        println!("move_player");
    }
    let dice_roll = 9 ;
    
    match dice_roll { 
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // 没有任何事情发生
    }
    println!(" ========================= ");
    let config_max =Some(3u8);
    match config_max {
        Some(max) => println!("max = {}",max),
        _ => (),
    }
    //等价于
    if let Some(max) = config_max{
        println!("max = {}",max)
    }else{ // 相当于match 最后的通用 other/_
        println!("max = None"); 
    }
}
