
use A12_1_IO::Config;
use std::env;
use std::process;

fn main() {
    println!(" 一个I/O项目 ：构建命令行程序");

    // 读取命令行参数值
    // std::env::args 再起任何参数无效Unicode字符时会panic
    // 如果需要接收Unicode 可以使用 std::env::args_os  返回值是OsString 值而不是String值。
    // let args :Vec<String> = env::args().collect();

    // let config  = parse_config(&args);
    // let config = Config::new(&args);
    // let config = Config::build(&args).unwrap_or_else(|error|{
    //     println!("Config error: {}", error);
    //     process::exit(1);
    // });

    // 使用第十三章的迭代器优化
    let config = Config::build(env::args()).unwrap_or_else(|error|{
        println!("Config error: {}", error);
        process::exit(1);
    });

    println!("query: {}", config.query);
    println!("file_path: {}", config.file_path);

    // 读取文件
    // let contents = fs::read_to_string(&config.file_path).expect("Something went wrong reading the file");
    // println!("contents:\n{}", contents);
    if let Err(e) =A12_1_IO::run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }

}
// 移动到 lib.rs里面
// struct Config{
//     query:String,
//     file_path :String,
// }
// impl Config {
//     // 优化方案 2
//     fn new(args :&[String]) -> Config {
//         if args.len() < 3{
//             panic!("Not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//         Config{query:query,file_path:file_path}
//     }
//
//     // 优化方案 3
//     fn build(args:&[String]) -> Result<Config,&'static str>{
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//         Ok(Config{query:query,file_path:file_path})
//     }
// }
//
// // 优化方案 1
// fn parse_config(args :&[String]) -> Config{
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//     Config { query, file_path }
// }
//
// fn run(config: Config) -> Result<(),Box<dyn Error>>{
//     let contents = fs::read_to_string(config.file_path)?;
//     println!("contents:\n{}", contents);
//     Ok(())
// }