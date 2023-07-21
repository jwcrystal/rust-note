use std::{env, fs};

fn main() {
    // 命令行輸入參數包含非 Unicode 字符，std::env::args 會直接崩潰
    // std::env::args_os 會引入跨平台複雜性，讓用戶自己處理參數錯誤
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    // let (query, filename) = parse_config(&args);
    // let config = parse_config(&args);
    let config = Config::new(&args);

    // let query = &args[1];
    // let file_path = &args[2];
    // // let mut file = std::fs::OpenOptions::new().read(true).open(file_path).unwrap();

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
    .expect("Should havbe been able to read the file.");//.unwrap();

    println!("With text:\n{}", contents);
}

/* 使用復雜類型更適合堅持使用基本類型，稱為基本類型偏執 (primitive obsession) 的反模式 (anti-pattern) */
// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
// }

struct Config {
    query: String,
    filename: String,
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
// }

impl Config {
    fn new (args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}