use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::path::PathBuf;

mod renderer;
pub mod cookie;
pub mod request;
pub mod response;

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
        let mut http_request = request::parse_request(request);
        let mut path = PathBuf::from(http_request.get_path());
        path.set_extension("");
        let path = path.as_os_str();
        let (view_function, pattern_map) = renderer::mapping_url_to_view(path);
        http_request.update_params(pattern_map);

        let mut http_response = view_function(&http_request);
        renderer::render(&mut http_response);
        let response = response::make_response(&http_request, &http_response);

        //リクエスト返信
        let _ = stream.write(&response);
        let _ = stream.shutdown(std::net::Shutdown::Both);
        println!("クライアントとの接続を終了します。address:{}", address);
    }
}