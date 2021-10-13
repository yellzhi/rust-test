
pub fn use_advanced_type(){
    // 高级类型
    println!("高级类型");

    // 类型别名用来创建类型同义词
    let x :i32 = 5;
    let y : MyI32 = 6;
    println!("x+y : {}", x+y);

    println!("// 从不返回的 never type ( fn ! )
       println!
       描述 ! 的行为的正式方式是 never type 可以强转为任何其他类型。
    ");


    println!("动态大小类型和 Sized trait");

}

type MyI32 = i32;

// 从不返回的 never type ( fn ! )