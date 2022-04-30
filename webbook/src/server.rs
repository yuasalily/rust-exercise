use std::net::{TcpListener, TcpStream};
use std::thread;

pub mod worker;
use worker::Worker;


pub struct TcpServer{
    addr: String,
}

impl TcpServer{
    pub fn new() -> Self {
        return TcpServer{
            addr: "172.17.0.3:80".to_string(),
        };
    }
    pub fn serve(&self){
        println!("サーバを起動します");
        
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            let (stream, address) = listener.accept().unwrap();
            thread::spawn(move || {
                let worker_thread = Worker::new();
                worker_thread.run(stream, address);
            });

        }
    }
}