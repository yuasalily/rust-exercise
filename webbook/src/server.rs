use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;
use std::fs;

pub mod cookie;



pub struct TcpServer{
}

impl TcpServer{
    pub fn serve(&self, addr:String){
        let listener = TcpListener::bind(addr).unwrap();
        let (mut stream, _) = listener.accept().unwrap();
        let mut file = File::create("server_recv.txt").unwrap();
        let mut buf = [0u8; 1024];
        let r = stream.read(&mut buf).unwrap();
        let _ = file.write_all(&buf[..r]);
        
        let mut file = File::open("server_send.txt").unwrap();
        let mut send_buf = Vec::new();
        let _ = file.read_to_end(&mut send_buf);
        println!("{:?}",send_buf);
        let _ = stream.write(&send_buf);
        let _ = stream.shutdown(std::net::Shutdown::Both);
    }


    pub fn sample(&self){
        println!("server");
        cookie::print_cookie();
    }
}