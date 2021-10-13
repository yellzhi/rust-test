use std::fs;
use std::error::Error;

pub struct Config{
    query : String,
    file_name : String,
}
impl Config{
    pub fn new(args : &[String]) ->Result<Config, &str>{
        if args.len() <3{
            return Err("not enough arguments")
        }
        let a1 = args[1].clone();
        let a2 = args[2].clone();
        Ok(Config {query: a1, file_name:a2})
    }

    pub fn new1( mut args : std::env::Args) -> Result<Config, &'static str>{
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let file_name = match args.next() {
            Some(arg)=> arg,
            None => return Err("Didn't get a file name")
        };

        Ok(Config{query, file_name})
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(&config.file_name)?;
    //println!("{}", content);
    let ret = search(&config.query, &content);
    println!("{:?}", ret);
    Ok(())
}

pub fn search<'a>(query:&'a str, contents: &'a str) -> Vec<&'a str>{
    // let mut ret = Vec::new();
    // for line in contents.lines(){
    //     if line.to_lowercase().contains(query.to_lowercase().trim()){
    //         ret.push(line)
    //     }
    // }
    // ret
    contents.lines().filter(|line|{
        line.contains(query) }).collect()
}