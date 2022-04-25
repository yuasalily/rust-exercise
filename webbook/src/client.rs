pub mod tcp_client {
    use std::net::{TcpListener, TcpStream};
    use std::io::{Read, Write};
    use std::fs::File;
    pub fn request(addr: String) {
        let mut stream = TcpStream::connect(addr).unwrap();
        let mut file = File::open("client_send.txt").unwrap();
        let mut send_buf = Vec::new();
        let mut recv_buf = [0u8; 8192];
        let _ = file.read_to_end(&mut send_buf);
        let _ = stream.write(&send_buf);
        let r = stream.read(&mut recv_buf).unwrap();

        let mut file = File::create("client_recv.txt").unwrap();
        let _ = file.write_all(&recv_buf[..r]);
        let _ = stream.shutdown(std::net::Shutdown::Both);
        
    }
}