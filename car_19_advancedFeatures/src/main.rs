
mod traits;
mod advancedType;
mod fnPointClose;
mod macros;

use std::slice;
use std::ops::Add;
use crate::traits::use_advanced_trait;

fn main() {
    println!("高级特性 unsafe trait type close default!");

    use_unsafe();

    println!("高级 trait");
    use_advanced_trait();

    advancedType::use_advanced_type();
    fnPointClose::use_fn_close();

    macros::use_macro();
}



fn use_unsafe(){
    //解引用裸指针
    // 调用不安全的函数或方法
    // 访问或修改可变静态变量
    // 实现不安全 trait
    // 访问 union 的字段

    // raw points
    println!("裸指针（raw pointers）");
    println!("允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
            不保证指向有效的内存
            允许为空
            不能实现任何自动清理功能");

    // 通过引用创建裸指针
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 : {}", *r1);
        println!("r2 : {}", *r2);
    }

    // use unsafe fn
    unsafe { dangerous()}
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = split_at_mut(r, 3);
    println!("a : {:?}", a);
    println!("b : {:?}", b);

}

unsafe fn dangerous() { println!("dangerous()")}

fn split_at_mut(slic: &mut [i32], mid : usize) ->(&mut [i32], &mut [i32]){
    let len = slic.len();
    let ptr = slic.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}

