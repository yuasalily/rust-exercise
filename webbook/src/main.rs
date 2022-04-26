// http://localhost:8080/

mod server;
mod client;

fn main() {
    let tcp_server =  server::TcpServer{};
    tcp_server.serve("172.17.0.2:80".to_string(), false);
    // client::tcp_client::request("127.0.0.1:80".to_string());
}