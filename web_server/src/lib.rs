use std::thread;
use std::sync::{mpsc, Arc, Mutex};

type Job= Box<dyn FnOnce(usize) + Send + 'static>;

enum Message{
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool{
    workers : Vec<Worker>,
    sender : mpsc::Sender<Message>,
}
impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{
        assert!(size>0);

        let (sender, reciver) = mpsc::channel();
        let reciver= Arc::new(Mutex::new(reciver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&reciver)));
        }
        ThreadPool{ workers, sender}
    }

    pub fn execute<F>(&self, f:F) where F : FnOnce(usize) + Send + 'static{
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
impl Drop for ThreadPool{
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


struct Worker{
    id : usize,
    thread : Option<thread::JoinHandle<()>>,
}
impl Worker{
    fn new(id:usize, reciver:Arc<Mutex<mpsc::Receiver<Message>>>) ->Worker{
        let thread = thread::spawn(move||{
            loop{
                let message = reciver.lock().unwrap().recv().unwrap();

                //job(id);
                match message {
                    Message::NewJob(job)=>{
                        println!("Worker {} got a job; executing.", id);
                        job(id);
                    }
                    Message::Terminate =>{
                        println!("Worker {} was told to terminate.", id);
                        break;
                    },
                }
            }
        });
        Worker{id, thread: Some(thread)}
    }
}