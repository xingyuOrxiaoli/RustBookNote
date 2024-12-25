use std::env;
use std::process;
use A12_2::Config;

fn main() {
    println!("将错误信息输出到标准错误而不是标准输出");

    let args:Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = A12_2::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}
