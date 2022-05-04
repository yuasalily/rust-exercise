use std::collections::HashMap;
use urlparse::parse_qs;

pub struct HttpRequest {
    path: String,
    method: String,
    http_version: String,
    body: HashMap<String, Vec<String>>,
    header: HashMap<String, String>,
    cookies: HashMap<String, String>,
    params: HashMap<String, String>,
}

impl HttpRequest {
    pub fn get_path(&self) -> String{self.path.clone()}
    pub fn get_method(&self) -> String{self.method.clone()}
    pub fn get_http_version(&self) -> String{self.http_version.clone()}
    pub fn get_body(&self) -> HashMap<String, Vec<String>>{self.body.clone()}
    pub fn get_header(&self) -> HashMap<String, String>{self.header.clone()}
    pub fn get_cookies(&self) -> HashMap<String, String>{self.cookies.clone()}
    pub fn get_params(&self) -> HashMap<String, String>{self.params.clone()}
    pub fn update_params(&mut self, map: HashMap<String, String>){self.params.extend(map);}

}


pub fn parse_request(request: Vec<u8>) -> HttpRequest{
    let request: String = String::from_utf8(request).unwrap();
    let request:Vec<String> = request.split("\r\n\r\n").map(|s| s.to_string()).collect();
    let (request_head, request_body) = (request.get(0).unwrap(), request.get(1).unwrap());
    let request_head:Vec<String> = request_head.split("\r\n").map(|s| s.to_string()).collect();
    let request_line: Vec<String> = request_head.get(0).unwrap().clone().split_whitespace().map(|s| s.to_string()).collect();
    let (method, path, http_version) = (request_line.get(0).unwrap(), request_line.get(1).unwrap(), request_line.get(2).unwrap());
    let request_body_map = parse_qs(request_body.clone());
    let mut header:HashMap<String, String> = HashMap::new();
    for i in 1..request_head.len(){
        let i = i as usize;
        let param = &request_head.get(i).unwrap().clone();
        let param:Vec<String> = param.split(":").map(|s| s.trim().to_string()).collect();
        header.insert(param.get(0).unwrap().clone(), param.get(1).unwrap().clone());
    }

    let cookies = header.get("Cookie");
    let cookies = match cookies {
        Some(cookie) => {
            let cookies = cookie.split(";").map(|s| {let s:Vec<String> = s.split("=").map(|t| t.trim().to_string()).collect();(s.get(0).unwrap().clone(),s.get(1).unwrap().clone())}).collect();
            cookies
        },
        None => {HashMap::<String, String>::new()}
    };
    let http_request = HttpRequest{
        path: path.clone(),
        method: method.clone(),
        http_version: http_version.clone(),
        body: request_body_map,
        header: header,
        cookies: cookies,
        params: HashMap::new(),
    };
    return http_request;
}