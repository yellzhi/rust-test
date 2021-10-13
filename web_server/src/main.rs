use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let thread = ThreadPool::new(5);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        //println!("Connection established!");
        thread.execute(|x|{
            handle_connection(stream, x)
        });
    }
}
fn handle_connection(mut stream:TcpStream, id : usize){
    let mut buff = [0; 1024];
    stream.read(&mut buff).unwrap();

    let txt = "HTTP/1.1 200 OK\r\n\r\n";
    let response = format!("{} {} {}",txt, " hello, thread:", id);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    //println!("Request: {}", String::from_utf8_lossy(&buff[..]));
}
