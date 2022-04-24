// http://localhost:8080/
use std::net::TcpListener;
use std::io::{Read, Write};
use std::fs::File;

struct TcpServer {
    addr:String,
}

impl TcpServer {
    fn serve(&self){
        let result = TcpListener::bind(&self.addr);
        match result {
            Result::Ok(listener) => {
                let res = listener.accept();
                match res {
                    Result::Ok((mut stream, _)) =>{
                        println!("{:?}", stream);
                        let mut file = File::create("server_recv.txt").unwrap();
                        let mut buf = [0u8; 1024];
                        let r = stream.read(&mut buf).unwrap();
                        let buf:String = buf[..r].iter().map(|c| *c as char).collect::<String>().replace("\r\n","\n");
                        // file.write_all(&buf);
                        let _ = write!(file, "{}", &buf);
                        let _ = stream.shutdown(std::net::Shutdown::Both);

                    },
                    Result::Err(e) => {
                        println!("Err");
                        println!("{:?}", e);
                    }
                }
            },
            Result::Err(e) => {
                println!("Err");
                println!("{:?}", e);
            },
        }
    }
}

fn main() {
    let tcp_server: TcpServer = TcpServer{addr:"172.17.0.2:80".to_string()};
    tcp_server.serve();
}