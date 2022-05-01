// http://localhost:8080/index
// http://localhost:8080/now
// http://localhost:8080/show_request
// http://localhost:8080/form

mod server;
mod settings;
// mod client;
// client::tcp_client::request("127.0.0.1:80".to_string());

fn main() {
    let tcp_server =  server::TcpServer::new();
    tcp_server.serve();
    
}