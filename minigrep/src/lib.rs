use std::{fs, error::Error, env};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // 原先的命名 new，相當意味著不會失敗建立實例，故改了名
    // pub fn build (args: &[String]) -> Result<Config, &'static str> {
    pub fn build (mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // panic!("not enough arguments, need 2 arguments.")
            return Err("not enough arguments, need 2 arguments.");
        }
        // 第一個參數為程序名，故空調用一次
        args.next();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };
        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file name."),
        };
        // 單純用 is_ok() 不嚴謹
        // let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        // 優先權 環境變量 > 命令參數
        let case_sensitive = env::var("CASE_INSENSITIVE").map_or_else(
            |_| {
            // 判斷命令參數，加入 "--case_insensitive" 則不檢查大小寫
            !args
                .any(|arg| arg.to_lowercase() == "--case_insensitive")
        }, |val| val == "0" || val.to_lowercase() == "false");

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let ret = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };

    for line in ret {
        println!("{}", line)
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut ret = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         ret.push(line);
    //     }
    // }
    
    // 引入 Iterator 
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_insensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut ret = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            ret.push(line)
        }
    }
    ret
}

/* Test Cases */
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "ast";
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

        assert_eq!(vec!["Rust:", "Trust me."], search_insensitive(query, contents));
    }
}