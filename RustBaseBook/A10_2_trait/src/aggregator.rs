use std::fmt::Display;

pub trait Summary{
    fn summarize(&self) ->String{
        String::from(" Summary summarize")
    }
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet{
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// trait作为参数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound语法
pub fn notify1<T :Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 通过 + 指定多个trait bound
pub fn notify2<T :Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify3(item: &(impl Summary + Display) ){
    println!("Breaking news! {}", item.summarize());
}

// 通过 where 简化
pub fn notify4<T>(item: &T)
where T: Display + Summary
{
    println!("Breaking news! {}", item.summarize());
}

// 返回值为 trait
pub fn notify5() -> impl Summary {
    Tweet {
        username: String::from("返回值为 trait: horse_ebooks   "),
        content: String::from("返回值为 trait : of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}