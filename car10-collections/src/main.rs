use std::collections::HashMap;

fn main() {
    test_vec();
    println!("*********************************************");

   // println!("Hello, world!");
    let mut v:Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];
    println!("{:?}", v1);

    v.push(1);
    v.push(3);
    v.push(567);
    v.push(34);
    println!("{:?}", v);

    let v2 = v[1];
    println!("{:?}", v);
    println!("v2 : {:?}", v2);

    // for i in &v1{
    //     // 遍历 vec
    // }
    // for i in &mut v1{
    //     // 遍历修改
    // }

    //枚举存储不同值
    #[derive(Debug)]
    enum Datatest {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        Datatest::Int(3),
        Datatest::Float(3.0),
        Datatest::Text(String::from("3.0")),
    ];
    for r in row{
        println!("{:?}", r)
    }

    // Rust 的核心语言中只有一种字符串类型：str，
    //字符串 slice，它通常以被借用的形式出现，&str

    println!("+++++++++++++++++++++++ String str+++++++++++++++++++++++++++++++++++++=");
    let mut s1 = String::from("foo");
    s1.push_str("bar");
    println!("{}",s1);
    s1.push('A');
    println!("{}",s1);

    let s2 = "hello".to_string();
    let s3 = String::from(" world");
     s1 = s2 + &s3;
    println!("{}",s1);
    s1 = format!("{}-{}", s1, s3);
    println!("{}",s1);

    println!("+++++++++++++++++++++++ Hash map+++++++++++++++++++++++++++++++++++++=");

    let mut scores = HashMap::new();
    scores.insert("blue", 10);
    scores.insert("yellow", 50);

    let teams = vec!["blue", "red"];
    let init_scores = vec![10,60];
    let scores1:HashMap<_,_> = teams.iter().zip(init_scores.iter()).collect();
    println!("{:?}",scores1);
    println!("{:?}", scores.get("blue"));
    for (k, v) in &scores{
        println!("{}-{}", k,v)
    }


}

fn test_vec(){
    let mut v = vec![1,5,87,43,656,7,87,43,1,5,5];
    v.sort();
    println!("{:?}",v);
    // averge
    let mut total =0;
    let  mut mapMode  = HashMap::new();
    for &i in &v{
        total +=i;
        let count = mapMode.entry(i).or_insert(0);
        *count +=1;
    };
    let aver = total/ v.len();
    let model = v[v.len()/2];

    println!("{:?}", mapMode)
}

