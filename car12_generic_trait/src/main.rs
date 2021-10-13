use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    //let p1 = PointTU{x:4, y:5};

    let arit = NewsArticle{
        headling:"news_heading".to_string(),
        location:"USA".to_string(),
        author: "bob".to_string(),
    };
    println!("{}", arit.summaroze());
    println!("{}", arit.default());

    notifyBound(&arit);
    notify(&arit);

    println!("比较最大值..........");
    let v1 = vec![1,4,5,6665,432,43,2,4,56,7];
    let ret1 = largest(&v1);
    println!("{}",ret1);
    let v2 = vec!['q', 'r', 'w', 'f'];
    println!("{}", largest(&v2));

    println!("{}", "*************使用 trait bound 有条件地实现方法***************");
    let p1 = Pair{x:3,y:4};
    p1.cmp_diplay();

}

struct PointTU<T>(T, T);

impl <T> PointTU<T>{
    fn get_x(&self) -> &T{
        &self.0
    }
}

// ***************************trait*********************************
pub trait Summary{
    fn summaroze(&self) ->String;
    fn default(&self) -> String{
        //默认实现
        return "default fun".to_string();
    }
}
pub struct NewsArticle{
    pub headling : String,
    pub location : String,
    pub author : String,
}
impl Summary for NewsArticle{
    fn summaroze(&self) -> String {
        format!("{} by {} ({})", self.headling, self.location, self.author)
    }
}

// 作为参数
pub fn notifyBound<T:Summary>(item:&T){
    println!("***********作为参数*******");
    println!("{}", item.summaroze());
    println!("{}", item.default());
}
pub fn notify(item:&impl Summary){
    println!("{}", "语法糖.......");
    println!("{}", item.default());

}

fn largest<T:PartialOrd> (list:&[T]) ->&T{
    let mut largest = &list[0];
    for item in list.iter(){
        if item > largest{
            largest = &item
        }
    }
    return largest
}

// *************使用 trait bound 有条件地实现方法***************
struct Pair<T>{
    x : T,
    y : T,
}
impl <T> Pair<T>{
    fn new(x:T, y:T) -> Self{
        Self{
            x, y,
        }
    }
}
impl <T:Display + PartialOrd> Pair<T>{
    fn cmp_diplay(&self){
        if self.x >self.y{
            println!("the lagest number is {}", self.x)
        }else {
            println!("the lagest number is {}", self.y)
        }
    }
}



