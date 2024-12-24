mod aggregator;

use aggregator::{Tweet,Summary};
use crate::aggregator::notify;

fn main() {
    println!("第十章 trait 部分 ");
    println!("疑惑点: 相干性 孤儿规则");
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!(" ====================  trait 作为参数 ===============================");
    notify(&tweet);

}
