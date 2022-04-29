use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;
use std::fs;
use chrono::Utc;
use std::path::PathBuf;
use std::env;

pub mod cookie;



pub struct TcpServer{
    static_dir: PathBuf,
    not_found_file_path: PathBuf,
    addr: String,
}

impl TcpServer{
    pub fn new() -> Self{
        let mut dir = env::current_dir().unwrap();
        dir.push("static");
        let mut not_found = dir.clone();
        not_found.push("404.html");
        return TcpServer{
            static_dir: dir,
            not_found_file_path: not_found,
            addr: "172.17.0.3:80".to_string(),
        }

    }
    pub fn serve(&self, save_recv_flg: bool){
        println!("サーバを起動します");
        let listener = TcpListener::bind(&self.addr).unwrap();
        let (mut stream, _) = listener.accept().unwrap();
        
        // クライアントからのリクエストを取得
        let mut request = [0u8; 1024];
        let r = stream.read(&mut request).unwrap();
        let request = request[..r].to_vec();
        
        if save_recv_flg {
            println!("クライアントのリクエストを保存します");
            let mut file = File::create("server_recv.txt").unwrap();
            let _ = file.write_all(&request);
        }
        let request: String = request.iter().map(|c| *c as char).collect();
        let request:Vec<String> = request.split("\r\n\r\n").map(|s| s.to_string()).collect();
        let (request_head, request_body) = (request.get(0).unwrap(), request.get(1).unwrap());
        let request_head:Vec<String> = request_head.split("\r\n").map(|s| s.to_string()).collect();
        let request_line: Vec<String> = request_head.get(0).unwrap().clone().split_whitespace().map(|s| s.to_string()).collect();
        let (method, path, http_version) = (request_line.get(0).unwrap(), request_line.get(1).unwrap(), request_line.get(2).unwrap());
        let path = path.strip_prefix("/").unwrap();
        
        let mut static_file_path = self.static_dir.clone();
        static_file_path.push(path);
        let file = File::open(&static_file_path);
        let (mut response, response_body) = match file {
            Ok(mut file) => {
                let mut body = Vec::new();
                let _ = file.read_to_end(&mut body);
                let body: String = String::from_utf8(body).unwrap();
                let line = "HTTP/1.1 200 OK\r\n".to_string();
                (line, body)
            }
            Err(_err) => {
                let mut body = Vec::new();
                let mut file = File::open(&self.not_found_file_path).unwrap();
                let _ = file.read_to_end(&mut body);
                let body: String = String::from_utf8(body).unwrap();
                let line = "HTTP/1.1 404 Not Found\r\n".to_string();
                (line, body)
            }
        };
        let mut response_head: String = chrono::Utc::now().format("Date: %a, %d %b %Y %H:%M:%S GMT\r\n").to_string();
        response_head += "Host: HenaServer/0.1\r\n";
        response_head += &format!("Content-Length: {}\r\n", response_body.len());
        response_head += "Connection: Close\r\n";
        response_head += "Content-Type: text/html\r\n";
        response_head += "\r\n";
        
        response.push_str(&response_head);
        response.push_str(&response_body);

        let response = response.as_bytes();
        
        let _ = stream.write(&response);
        let _ = stream.shutdown(std::net::Shutdown::Both);
    }


    pub fn _get_apache_text(&self) -> Vec<u8> {
        println!("apacheのレスポンスとのコピーを取得");
        let mut file = File::open("server_send.txt").unwrap();
        let mut send_buf = Vec::new();
        let _ = file.read_to_end(&mut send_buf);
        
        cookie::_print_cookie();
        return send_buf;
    }
}