use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    let handle = std::thread::spawn(|| {
        for i in 1..4{
            println!("thead:{}, run", i);
            thread::sleep(Duration::from_millis(20));
        }
    });
    //thread::sleep(Duration::from_secs(3));
    handle.join().unwrap();

    test_channel();

    use_mutex1();

}

fn test_channel(){

    let (tx, rx) = mpsc::channel();
    let v1 = vec![1,2,4,5];
    let handle1 = thread::spawn(move||{
        println!("{:?}", v1);
        tx.send(v1[0]).unwrap();
        let tx1 = tx.clone();
        for v in v1{
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_millis(500));
        }

    });
    let re = rx.recv().unwrap();
    println!("thread recv: {}", re);
    //handle1.join().unwrap();
    for r in rx{
        println!("thread recv: {}", r);
    }
}

fn test_mutex(){


}
fn use_mutex1(){
    // Rc 引用技术 线程不安全
    // Arc 原子行，线程安全
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();
            *num +=1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}
