
// 宏
#[macro_export]
macro_rules! sqr {
    ($x:expr) => { $x * $x };
}

#[macro_export]
macro_rules! add {
    // 2
    ($a:expr, $b:expr) => { {$a + $b} };
    // 1
    ($a:expr) => {$a};
    // more
    ($a:expr, $($b:tt)*)=>{
        $a + add!($($b)*)
    }
}

pub fn use_macro(){
    println!("因为宏模式所匹配的是 Rust 代码结构而不是值");
    println!("{}", add!(5,6));
    println!("{}", add!(5));
    println!("{}", sqr!(5));
    println!("{}", add!(5,5,6,7,9))
}

