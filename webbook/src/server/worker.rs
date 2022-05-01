use std::net::{TcpStream, SocketAddr};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::fs::File;
use chrono::Utc;
use std::path::{PathBuf, Path};

mod view;
mod request;
mod response;
mod urls;

pub struct Worker{

}

impl Worker{
    pub fn new() -> Self {
        return Worker{};
    }

    // クライアントからのリクエストを取得
    pub fn run(&self, mut stream:TcpStream, address:SocketAddr){
        println!("クライアントとの接続が完了しました。address:{}", address);
        // リクエスト読み込み
        let mut request = [0u8; 1024];
        let r = stream.read(&mut request).unwrap();
        let request = request[..r].to_vec();
        
        // レクエスト処理
        let parsed_request = request::parse_request(request);
        let path = Path::new(parsed_request.get("path").unwrap()).file_stem().unwrap();
        let view_function = urls::mapping_url_to_view(path);

        let (response_line, response_body, extension) = view_function(&parsed_request);
        let response = response::make_response(response_line, response_body, &extension);

        //リクエスト返信
        let _ = stream.write(&response);
        let _ = stream.shutdown(std::net::Shutdown::Both);
        println!("クライアントとの接続を終了します。address:{}", address);
    }
}