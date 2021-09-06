fn main() {
   // println!("Hello, world!");
    let mut v:Vec<i32> = Vec::new();
    let mut v1 = vec![1, 2, 3];
    println!("{:?}", v1);

    v.push(1);
    v.push(3);
    v.push(567);
    v.push(34);
    println!("{:?}", v);

    let v2 = v[1];
    println!("{:?}", v);
    println!("v2 : {:?}", v2);

    for i in &v1{
        // 遍历 vec
    }
    for i in &mut v1{
        // 遍历修改
    }

    // Rust 的核心语言中只有一种字符串类型：str，
    //字符串 slice，它通常以被借用的形式出现，&str

}

