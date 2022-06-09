use std::env;
use std::process;

use self::core::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("解析参数异常:{}",err);
        process::exit(1);
    });

    println!("检索字符串:{}", config.query);
    println!("检索文件:{}", config.filename);

    // let contents = fs::read_to_string(filename).expect("读文件错误");
    if let Err(e) = core::run(config) {
        eprintln!("程序错误:{}",e);
        process::exit(1);
    }
    

    // println!("{:?}",args);
    // println!("Hello, world!");
}




