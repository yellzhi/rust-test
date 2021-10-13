use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::fmt::{Display, Formatter};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    println!("Hello, world!");

    let b = Box::new(5);
    println!("b={}, *b={}", b, *b);

    let list = Cons(1,
                    Box::new(Cons(2,
                        Box::new(Cons(3,
                            Box::new(Nil))))));

    println!("{:?}", list);

    println!("Deref trait .......");
    let b = MyBox::new(7);
    println!("b={}, *b={}", b, *b);


    test_rc_refcell()


}
#[derive(Debug)]
enum List{
    Cons(i32, Box<List>),
    Nil,
}


struct MyBox<T>(T);
impl<T>  MyBox<T> {
    fn new(x: T) ->MyBox<T>{
        MyBox(x)
    }
}
impl <T:Display> Display for MyBox<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&**self, f)
    }
}
impl <T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 多共享 可修改
#[derive(Debug)]
enum List1{
    Cons1 (Rc<RefCell<i32>>, Rc<List1>),
    Nil1,
}
use crate::List1::{Cons1, Nil1};
fn test_rc_refcell(){
    let value =Rc::new(RefCell::new(5));
    let a = Rc::new(Cons1(Rc::clone(&value), Rc::new(Nil1)));
    let b = Cons1(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons1(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}


