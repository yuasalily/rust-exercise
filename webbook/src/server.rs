use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;
use std::fs;
use chrono::Utc;

pub mod cookie;



pub struct TcpServer{
}

impl TcpServer{
    pub fn serve(&self, addr:String, save_recv_flg: bool){
        println!("サーバを起動します");
        let listener = TcpListener::bind(addr).unwrap();
        let (mut stream, _) = listener.accept().unwrap();
        
        // クライアントからのレクエストを取得
        let mut request = [0u8; 1024];
        let r = stream.read(&mut request).unwrap();
        let request = request[..r].to_vec();
        
        if save_recv_flg {
            println!("クライアントのリクエストを保存します");
            let mut file = File::create("server_recv.txt").unwrap();
            let _ = file.write_all(&request);
        }
        let mut response: String = "HTTP/1.1 200 OK\r\n".to_string(); 
        let mut response_body: String = "<html><body><h1>It works!</h1></body></html>".to_string();

        let mut response_head: String = chrono::Utc::now().format("Date: %a, %d %b %Y %H:%M:%S GMT\r\n").to_string();
        response_head += "Host: HenaServer/0.1\r\n";
        response_head += &format!("Content-Length: {}\r\n", response_body.as_bytes().len());
        response_head += "Connection: Close\r\n";
        response_head += "Content-Type: text/html\r\n";
        response_head += "\r\n";
        
        response.push_str(&response_head);
        response.push_str(&response_body);

        
        println!("{:?}", response);
        let response = response.as_bytes();
        // let response = self._get_apache_text();
        
        let _ = stream.write(&response);
        let _ = stream.shutdown(std::net::Shutdown::Both);
    }


    pub fn _get_apache_text(&self) -> Vec<u8> {
        println!("apacheのレスポンスとのコピーを取得");
        let mut file = File::open("server_send.txt").unwrap();
        let mut send_buf = Vec::new();
        let _ = file.read_to_end(&mut send_buf);
        
        cookie::print_cookie();
        return send_buf;
    }
}