
use crate::garden::vegetables::Asparagus;
pub mod garden;


fn main() {
    println!("第七章 使用包、Crate和模块管理");
    
    println!("crate crate-root package 包含一个Cargo.toml");
    println!("包中可以包含至多一个库crate  可以有多个二进制crate  但是总体来说至少包含一个crate");
    println!("模块、路劲、use关键字和pub关键字");
    println!("========== Asparagus ==========");
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
    
    
}
