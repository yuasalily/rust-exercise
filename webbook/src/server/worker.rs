use std::net::{TcpStream, SocketAddr};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::fs::File;
use chrono::Utc;
use std::path::PathBuf;
use std::env;

pub struct Worker{
    static_dir: PathBuf,
    not_found_file_path: PathBuf,
}

impl Worker{
    pub fn new() -> Self {
        let mut dir = env::current_dir().unwrap();
        dir.push("static");
        let mut not_found = dir.clone();
        not_found.push("404.html");
        return Worker{
            static_dir: dir,
            not_found_file_path: not_found,
        };
    }

    // クライアントからのリクエストを取得
    pub fn run(&self, mut stream:TcpStream, address:SocketAddr){
        println!("クライアントとの接続が完了しました。address:{}", address);
        // リクエスト読み込み
        let mut request = [0u8; 1024];
        let r = stream.read(&mut request).unwrap();
        let request = request[..r].to_vec();
        
        // レクエスト処理
        let parsed_request = Worker::parse_request(request);
        let path = parsed_request.get("path").unwrap();
        let mut static_file_path = self.static_dir.clone();
        static_file_path.push(&path);
        let extension = &static_file_path.as_path().extension().unwrap().to_str().unwrap();
        let (response, response_body) = self.get_response_content(&static_file_path);
        let response = Worker::make_response(response, response_body, extension);

        //リクエスト返信
        let _ = stream.write(&response);
        let _ = stream.shutdown(std::net::Shutdown::Both);
        println!("クライアントとの接続を終了します。address:{}", address);
    }
    fn get_response_content(&self, path: &PathBuf) -> (String, Vec<u8>){
        let file = File::open(&path);
        let (response, response_body) = match file {
            Ok(mut file) => {
                let mut body = Vec::new();
                let _ = file.read_to_end(&mut body);
                let line = "HTTP/1.1 200 OK\r\n".to_string();
                (line, body)
            }
            Err(_err) => {
                let mut body = Vec::new();
                let mut file = File::open(&self.not_found_file_path).unwrap();
                let _ = file.read_to_end(&mut body);
                let line = "HTTP/1.1 404 Not Found\r\n".to_string();
                (line, body)
            }
        };
        return (response, response_body);
    }

    fn make_response(mut response: String, response_body:Vec<u8>, extension: &str) -> Vec<u8>{
        let mut response_head: String = Utc::now().format("Date: %a, %d %b %Y %H:%M:%S GMT\r\n").to_string();
        response_head += "Host: HenaServer/0.1\r\n";
        response_head += &format!("Content-Length: {}\r\n", response_body.len());
        response_head += "Connection: Close\r\n";
        response_head += &format!("Content-Type: {}\r\n", Worker::get_mime_type_from_extension(extension));
        response_head += "\r\n";
        
        response.push_str(&response_head);

        let mut response = response.as_bytes().to_vec();
        response.extend(response_body);
        return response;
    }

    fn parse_request(request: Vec<u8>) -> HashMap<String, String>{
        let mut parsed_request: HashMap<String,String> = HashMap::new();
        let request: String = request.iter().map(|c| *c as char).collect();
        let request:Vec<String> = request.split("\r\n\r\n").map(|s| s.to_string()).collect();
        let (request_head, request_body) = (request.get(0).unwrap(), request.get(1).unwrap());
        let request_head:Vec<String> = request_head.split("\r\n").map(|s| s.to_string()).collect();
        let request_line: Vec<String> = request_head.get(0).unwrap().clone().split_whitespace().map(|s| s.to_string()).collect();
        let (method, path, http_version) = (request_line.get(0).unwrap(), request_line.get(1).unwrap(), request_line.get(2).unwrap());
        let path = path.strip_prefix("/").unwrap().to_string();

        parsed_request.insert("path".to_string(), path);

        return parsed_request;
    }

    fn get_mime_type_from_extension(extension: &str) -> String{
        match extension {
            "html" => "text/html".to_string(),
            "css" => "text/css".to_string(),
            "png" => "image/png".to_string(),
            "jpg" => "image/jpg".to_string(),
            "gif" => "image/gif".to_string(),
            _ => "application/octet-stream".to_string(),
        }
    }
    
}