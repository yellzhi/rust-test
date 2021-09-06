fn main() {
    println!("Hello, world!");

    //创建 User 结构体的实例
    let u1 = User{
        name: String::from("test"),
        email:String::from("test.com"),
        age : 20
    };

    println!("---------struct--------");
    println!("u1: {}",u1.email);

    //  u2 中创建一个新 User 实例 name 设置了新的值 其他值则使用了实例 u1 中的同名值：
    let u2 = User{
        name:String::from("test2"),
        ..u1
    };

    println!("u2: {:#?}", u2);

    println!("------struct func-------");
    let r1 = Rectangle{w:34, h:3};
    println!("the eara is : {}", r1.area());
    let r2 = Rectangle{w:33, h:2};
    println!("r1 can hold r2 ? {}", r1.can_hold(&r2));

    let s1 = Rectangle::square(22);
    println!("------- 正方形--------");
    println!("{:#?}",s1);
}

// 增加注解来派生 Debug trait，并使用调试格式打印 User 实例
#[derive(Debug)]
struct User{
    name : String,
    email : String,
    age : u16,
}

//元组
struct color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle{
    w : u32,
    h : u32,
}
impl Rectangle{
    // &self，做出修改 &mut self 或者是获取所有权 self
    fn area(&self) ->u32{
        self.h * self.w
    }
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.w > other.w && self.h > other.h
    }

    //构造正方形
    fn square(size: u32) -> Rectangle{
        Rectangle{w:size, h:size}
    }
}



