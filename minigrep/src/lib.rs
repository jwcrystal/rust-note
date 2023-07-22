use std::{fs, error::Error};


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // .expect("Should havbe been able to read the file.");

    println!("With text:\n{}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // 原先的命名 new，相當意味著不會失敗建立實例，故改了名
    pub fn build (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // panic!("not enough arguments, need 2 arguments.")
            return Err("not enough arguments, need 2 arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}