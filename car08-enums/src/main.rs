fn main() {
    println!("------enums------");
    let addr1 = IpAddr{kind:IpKind::V6, addr:String::from("::1")};
    println!("{:#?}", addr1);

    let red = Color::Red;
    println!("{:?}", red);

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    test_match();

    println!("---------if let --------");
    let num =1;
    if let 5=num{
        println!("ok");
    }else {
        println!("faild")
    }

}

#[derive(Debug)]
enum IpKind{
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr{
    kind : IpKind,
    addr : String
}

// 可以将任意类型的数据放入枚举成员中：例如字符串、数字类型或者结构体。甚至可以包含另一个枚举
#[derive(Debug)]
enum IpAddr2{
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Color{
    Red,
    Black,
    Yellow,
    Blue,
}


// match

fn test_match(){
    println!("------match-------");
    println!("{}", value_in(Color::Black));
    println!("{}", value_in(Color::Red));
}
fn value_in(color: Color) ->u8{
    match color {
        Color::Red =>{
            println!("Luck !");
            1
        },
        Color::Black =>2,
        Color::Yellow=>3,
        _ => {0}
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None =>None,
        Some(i)=>Some(i+1),
    }
}
