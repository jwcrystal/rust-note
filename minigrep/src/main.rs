use std::{env, process};

use minigrep::{Config, run};

fn main() {
    // 命令行輸入參數包含非 Unicode 字符，std::env::args 會直接崩潰
    // std::env::args_os 會引入跨平台複雜性，讓用戶自己處理參數錯誤
    // let args: Vec<String> = env::args().collect();
    // // dbg!(&args);

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing: {}", err);
    //     process::exit(1)
    // });

    let config = Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // run(Config) 有可能有 Error，故需要處理錯誤情況
    if let Err(e) = run(config) {
        eprintln!("Error: {}",e);
        process::exit(1);
    };
}