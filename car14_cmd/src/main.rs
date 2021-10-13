use std::{env, process};
use car14_cmd::{Config, run};

fn main() {
    println!("接受命令行参数");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    //let fileName = &args[1];
    //let config = Config::new1(&args).unwrap_or_else(|err|{
    let config = Config::new1(env::args()).unwrap_or_else(|err|{
        println!("{}", err);
        process::exit(1);
    });
    if let Err(e) = run(&config){
        println!("Application error: {}", e);
        process::exit(1);
    }
}
