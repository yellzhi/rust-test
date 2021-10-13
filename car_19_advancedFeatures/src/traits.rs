
pub fn use_advanced_trait(){
    let p1 = Point{x:5,y:7};
    let p2 = Point{x:50,y:2};
    println!("p1 +p2 : {}", p1+p2);

    println!("完全限定语法与消歧义：调用相同名称的方法");
    println!("<Type as Trait>::function(receiver_if_method, next_arg, ...);");
    let person = Human{};
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("父 trait 用于在另一个 trait 中使用某 trait 的功能");
    let p3 = Point{x:5,y:7};
    p3.outline_print();

}
struct Point{
    x:i32, y:i32,
}
impl Add for Point{
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point{ x : self.x + other.x, y : self.y + other.y, }
    }
}

// 毫米值与米值相加
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters{
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0*1000))
    }
}

//完全限定语法与消歧义：调用相同名称的方法
trait Pilot{ fn fly(&self); }
trait Wizard { fn fly(&self); }
struct Human;
impl Pilot for Human{
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human{
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human{
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// 父 trait 用于在另一个 trait 中使用某 trait 的功能
use std::fmt;
use std::fmt::Formatter;
use std::ops::Add;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
impl OutlinePrint for Point{}
impl fmt::Display for Point{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}