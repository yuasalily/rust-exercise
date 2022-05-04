// http://localhost:8080/index
// http://localhost:8080/now.html
// http://localhost:8080/show_request
// http://localhost:8080/form
// http://localhost:8080/not_found
// http://localhost:8080/user/1123/profile
// http://localhost:8080/login
// http://localhost:8080/welcome

mod server;
mod settings;

fn main() {
    let tcp_server = server::TcpServer::new();
    tcp_server.serve();
    
}