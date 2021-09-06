use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::process::exit;

fn main() {
    println!("Guss the num1");
    println!(" Input your guss num:");
    let mut guess = String::new(); // mut 可变变量
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!(" You guess:{}", guess);

    // 引入 rand
    let num =rand::thread_rng().gen_range(1..10);
    println!("rand:{}", num);
    // let guess:u32 = guess.trim().parse()
    //     .expect("please input a num");
    let  guess:u32= match guess.trim().parse(){
        Ok(num) =>num,
        Err(_)=>0,
    };
   let ret= match guess.cmp(&num) {
        Ordering::Less => {"small"},
        Ordering::Equal => {"ok"},
        Ordering::Greater => {"big"},
    };
    println!("比较结果:{}",ret);


    println!("func ... :");
    println!("{}",add(4,5));
}

fn add (x:u32, y:u32) ->u32{
    let ret =x+y;
    ret
}
