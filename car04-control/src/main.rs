fn main() {
    println!("Hello, world!");

    // if
    let number = 3;
    if  number <5{
        println!("smarll")
    }else {
        println!("big")
    }

    let c = true;
    let n1 = if c{
        5
    }else { 6 };
    println!("n1:{}", n1);

    //loop
    println!("----------loop----------");
    let mut count = 0;
    'count_up:loop{
     println!("count = {}", count);
        let mut rem  =10;
        loop{
            println!("rem : {}", rem);
            if rem == 9{
                break;
            }
            if count == 2{
                break 'count_up
            }
            rem -=1;
        }
        count +=1
    }
    println!("end count:{}", count);

    // 从循环 返回
    println!("-------从循环 返回--------");
    let mut counter = 0;
    let ret = loop{
        counter += 1;
        if counter ==10{
            break counter*2;
        }
    };
    println!("ret : {}", ret);

    println!("--------while-----------");
    let mut n2 =2;
    while n2!=0 {
        println!("n2={}", n2);
        n2 -=1;
    }

    println!("----------for--------------");
    let a = [10,323,433,54,5];
    for e in a.iter(){
        println!("value is : {}", e);
    }
    println!("test 2...");
    for n in (1..4).rev(){
        println!("value is : {}", n);
    }
}
