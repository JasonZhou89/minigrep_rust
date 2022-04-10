use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        //这里clone可以进行优化
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

//错误时，返回实现了 Error trait 的类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //读取文件,expect会返回panic
    // let contents = fs::read_to_string(config.filename)
    // .expect("something went wrong reading the file");

    //不同于panic!,? 会从函数中返回错误值并让调用者处理
    let contents = fs::read_to_string(config.filename)?;
    
    for line in search(&config.query, &contents){
        println!("{}",line);
    }

    Ok(())
}

pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    // vec![]
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }
}