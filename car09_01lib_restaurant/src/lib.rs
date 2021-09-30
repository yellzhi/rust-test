mod front_of_hous{
   pub mod hosting{
        pub fn add_to_waitlist(){
            println!("add_to_waitlist")
        }
        fn seat_at_table(){}
    }
    mod serving{
        fn take_order(){}
        fn server_order() {}
        fn take_payment() {}
    }
}
fn save_order(){}
use crate::front_of_hous::hosting::add_to_waitlist;

pub fn eat_at_restaurant(){
    // 绝对路径
    crate::front_of_hous::hosting::add_to_waitlist();
    // 相对路径
    front_of_hous::hosting::add_to_waitlist();
    // use
    add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_host::Breakfase::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast)

}

mod back_of_host{
    fn cook_order(){
        println!("cook_order")
    }
    fn fix_incorrect_order(){
        cook_order();
        super::save_order();
    }
    // 例子模拟的情况是，在一家餐馆中，顾客可以选择随餐附赠的面包类型，
    //但是厨师会根据季节和库存情况来决定随餐搭配的水果。餐馆可用的水果变化是很快的，
    //所以顾客不能选择水果，甚至无法看到他们将会得到什么水果
    pub struct Breakfase{
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Beakdast{
        pub fn summer(toast:&str)->Breakfase{
            Breakfase{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches")
            }
        }
    }
}

//
