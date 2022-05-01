use std::collections::HashMap;
use chrono::Utc;

pub fn parse_request(request: Vec<u8>) -> HashMap<String, String>{
    let mut parsed_request: HashMap<String,String> = HashMap::new();
    let request: String = String::from_utf8(request).unwrap();
    let request:Vec<String> = request.split("\r\n\r\n").map(|s| s.to_string()).collect();
    let (request_head, request_body) = (request.get(0).unwrap(), request.get(1).unwrap());
    let request_head:Vec<String> = request_head.split("\r\n").map(|s| s.to_string()).collect();
    let request_line: Vec<String> = request_head.get(0).unwrap().clone().split_whitespace().map(|s| s.to_string()).collect();
    let (method, path, http_version) = (request_line.get(0).unwrap(), request_line.get(1).unwrap(), request_line.get(2).unwrap());
    let real_path = path.clone();
    let path = path.strip_prefix("/").unwrap().to_string();
    let method = method.clone();
    let http_version = http_version.clone();
    let request_body = request_body.clone();
    let mut header = String::new();
    for i in 1..request_head.len(){
        let i = i as usize;
        header += &request_head.get(i).unwrap().clone();
        header += "\r\n";
    }

    //使いたいパラメータが増えたらここを増やす
    parsed_request.insert("path".to_string(), path);
    parsed_request.insert("real_path".to_string(), real_path);
    parsed_request.insert("method".to_string(), method);
    parsed_request.insert("http_version".to_string(), http_version);
    parsed_request.insert("request_body".to_string(), request_body);
    parsed_request.insert("request_header".to_string(), header);
    parsed_request.insert("now".to_string(), Utc::now().format("Date: %a, %d %b %Y %H:%M:%S GMT\r\n").to_string());

    return parsed_request;
}