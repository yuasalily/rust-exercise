use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("?????").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // 接続が確立しました
        println!("Connection established!");
    }
}