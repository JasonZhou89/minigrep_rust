use minigrep::Config;
use std::env;
use std::process;
fn main() {
    //命令行参数获取
    let args: Vec<String> = env::args().collect();
    // let query = &args[1];
    // let filename = &args[2];
    // let (query,filename) = parse_config(&args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments:{}", err);
        process::exit(1);
    });
    println!("searching for {} ", config.query);
    println!("in file {}", config.filename);

    //这里没有使用unwrap_or_else，因为这里只关心错误情况，正确只会返回()没有实际意义
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error:{}", e);
        process::exit(1);
    }
}

// fn parse_config(args:&[String])->(&str,&str){
//     let query = &args[1];
//     let filename = &args[2];
//     (query,filename)
// }
