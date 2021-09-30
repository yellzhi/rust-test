use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;
use std::f32::consts::E;

fn main() {
    println!("Hello, world!");
    let f = File::open("hello.txt");
    let f =match  f{
        Ok(file) =>file,
        Err(err) => match  err.kind(){
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fs) =>fs,
            Err(e) =>panic!("Problem creating the file: {:?}", e),
        },
            other=>panic!("Problem opening the file: {:?}", other),
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
        });

    let ret = read_name_from_file();

}

fn read_name_from_file() ->Result<String, io::Error>{
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(fs)=>fs,
        Err(e) =>e,
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) =>Ok(s),
        Err(e) =>Err(e),
    }
}

fn read_name_from_fileV1() ->Result<String, io::Error>{
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}