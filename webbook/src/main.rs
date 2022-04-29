// http://localhost:8080/indx.html

mod server;
mod client;

fn main() {
    let tcp_server =  server::TcpServer::new();
    tcp_server.serve(false);
    // client::tcp_client::request("127.0.0.1:80".to_string());
}