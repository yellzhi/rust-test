use std::thread;
use std::time::Duration;

fn main() {
    println!("闭包........");

    let close = |num|{
        println!("sleep {} sec", num);
        thread::sleep(Duration::from_secs(num));
        num
    };
    //close(2);
    workout();

    iterTest()
}

fn workout(){
    let mut ret = Cacher::new(|num|{
        println!("sleep {} sec", num);
        thread::sleep(Duration::from_secs(num as u64));
        num +2
    });

    println!("{:?}", ret.value(1));
    println!("{:?}", ret.value(2));
    println!("{:?}", ret.value(3));
}

struct Cacher<T>
    where T : Fn(u32) ->u32 {
    calculation: T,
    value: Option<u32>,
}
impl <T> Cacher<T> where T:Fn(u32) -> u32{
    fn new(calculation: T) -> Cacher<T>{
        Cacher{
            calculation,
            value:None,
        }
    }
    fn value(&mut self, arg: u32) -> u32{
        match self.value {
            Some(v) => v,
            None => {
                let v= (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn iterTest(){
    let v1 = vec![1,2,3];
    let v2:Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("{:?}", v2);

    let v3 : Vec<_> = v1.iter().filter(|x| x ==&&2).collect();
    println!("{:?}", v3);

}


