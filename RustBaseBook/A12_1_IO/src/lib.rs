
use std::error::Error;
use std::{env, fs};

pub struct Config{
    pub query:String,
    pub file_path:String,
    pub ignore_case:bool,
}
impl Config{
    // 使用第十三章中的迭代器优化后的build代码
    pub fn build(
        mut args : impl Iterator<Item=String>,
    ) -> Result<Config, &'static str>{
        // 使用 Iterator trait 代替索引
        args.next();
        
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Missing query string"),
        };
        
        let file_path = match args.next(){
            Some(arg) => arg,
            None => return Err("Missing file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config{query,file_path,ignore_case})
    }
    // pub fn build(args : &[String]) -> Result<Config, &'static str>{
    //     if args.len() < 3 {
    //         return Err("not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();
    //     
    //     let ignore_case = env::var("IGNORE_CASE").is_ok();
    //     Ok(Config{query,file_path,ignore_case})
    // }
}

pub fn run(config: Config) ->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query , &contents) {
        println!("{}", line);
    }
    Ok(())
}
//  使用迭代器适配器来使代码更简明
pub fn search<'a> (query:&str,contents:&'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
// pub fn search<'a> (query:&str,contents:&'a str) -> Vec<&'a str>{
//     let mut results = Vec::new();
//     for line in contents.lines(){
//         if line.contains(query){
//             results.push(line);
//         }
//     }
//     results
// }
// 实现大小写不敏感
pub fn search_case_insensitive<'a>(
    query:&str,
    contents:&'a str,
) -> Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}