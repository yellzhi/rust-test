

pub(crate) fn use_fn_close(){
    println!("高级函数与闭包");

    println!("函数指针");
    let func = do_twice(add_one, 1);
    println!("{}", func);

    println!("返回闭包");



}
fn add_one(x:i32) -> i32{
    x +1
}
fn do_twice(f: fn(i32)->i32, val : i32) -> i32{
    f(val) + f(val)
}

// 返回闭包
fn result_close() -> Box<dyn Fn(i32)->i32>{
    //Box::new(add_one)
    Box::new(|x|x+1)
}
