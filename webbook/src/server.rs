use std::net::{TcpListener, TcpStream};
use std::thread;

pub mod worker;
use worker::Worker;
use crate::settings;

pub struct TcpServer{
}

impl TcpServer{
    pub fn new() -> Self {
        return TcpServer{};
    }
    pub fn serve(&self){
        println!("サーバを起動します");
        
        let listener = TcpListener::bind(settings::ADDRESS).unwrap();
        loop {
            let (stream, address) = listener.accept().unwrap();
            thread::spawn(move || {
                let worker_thread = Worker::new();
                worker_thread.run(stream, address);
            });

        }
    }
}