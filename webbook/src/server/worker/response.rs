use chrono::Utc;
use std::collections::HashMap;
use crate::server::worker::cookie::Cookie;
use crate::server::worker::request::HttpRequest;

pub struct HttpResponse{
    status_code: u32,
    header_line: String,
    header: HashMap<String, String>,
    cookies: Vec<Cookie>,
    extension: String,
    params: HashMap<String, String>,
    file_name: String,
    body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(file_name:String) -> HttpResponse{
        HttpResponse{
            status_code: 200,
            header_line: String::new(),
            header: HashMap::new(),
            cookies: Vec::new(),
            extension:String::new(),
            params: HashMap::new(),
            body: Vec::new(),
            file_name:file_name,
        }
    }
    pub fn get_file_name(&self) -> String{return self.file_name.clone();}
    pub fn get_status_code(&self) -> u32{return self.status_code;}
    pub fn get_params(&self) -> HashMap<String, String>{return self.params.clone();}
    pub fn get_body(&self) -> Vec<u8>{return self.body.clone();}
    pub fn get_extension(&self) -> String{return self.extension.clone();}
    pub fn get_header_line(&self) -> String{return self.header_line.clone();}
    pub fn get_header(&self) -> HashMap<String, String>{return self.header.clone();}
    pub fn get_cookies(&self) -> &Vec<Cookie>{return &self.cookies;}
    pub fn set_status_code(&mut self, status_code:u32){self.status_code = status_code;}
    pub fn set_body(&mut self, body:Vec<u8>){self.body = body;}
    pub fn set_header_line(&mut self, header_line:String){self.header_line = header_line;}
    pub fn set_extension(&mut self, extension: String){self.extension = extension;}
    pub fn update_header(&mut self, header:HashMap<String, String>){self.header.extend(header);}
    pub fn update_params(&mut self, params:HashMap<String, String>){self.params.extend(params.clone());}
    pub fn add_cookie(&mut self, cookie:Cookie){self.cookies.push(cookie);}
}

pub fn make_response(request: &HttpRequest, response: &HttpResponse) -> Vec<u8>{
    let response_body = response.get_body();
    let extension = response.get_extension();
    let mut response_line = response.get_header_line();
    let mut response_head: String = Utc::now().format("Date: %a, %d %b %Y %H:%M:%S GMT\r\n").to_string();
    response_head += "Host: HenaServer/0.1\r\n";
    response_head += &format!("Content-Length: {}\r\n", response_body.len());
    response_head += "Connection: Close\r\n";
    response_head += &format!("Content-Type: {}\r\n", get_mime_type_from_extension(&extension));
    for (k, v) in response.get_header(){
        response_head += &format!("{}: {}\r\n", k, v);
    }
    for cookie in response.get_cookies(){
        let mut cookie_line: String = format!("Set-Cookie: {}={}",cookie.get_name(),cookie.get_value());
        match cookie.get_expires() {
            Some(expires) => {cookie_line += &expires.format("; Expires=%a, %d %b %Y %H:%M:%S GMT").to_string();},
            None => {}
        }
        match cookie.get_max_age() {
            Some(max_age) => {cookie_line += &format!("; Max-Age={}", max_age);},
            None => {}
        }
        if !cookie.get_domain().is_empty() {
            cookie_line += &format!("; Domain={}", cookie.get_domain());
        }
        if !cookie.get_path().is_empty() {
            cookie_line += &format!("; Path={}",cookie.get_path());
        }
        if cookie.get_secure() {
            cookie_line += "; Secure";
        }
        if cookie.get_http_only() {
            cookie_line += "; HttpOnly";
        }
        cookie_line += "\r\n";
        response_head += &cookie_line;
    }
    response_head += "\r\n";
    response_line.push_str(&response_head);

    let mut response_line = response_line.as_bytes().to_vec();
    response_line.extend(response_body);
    return response_line;
}

fn get_mime_type_from_extension(extension: &str) -> String{
    match extension {
        "html" => "text/html; charset=UTF-8".to_string(),
        "css" => "text/css".to_string(),
        "png" => "image/png".to_string(),
        "jpg" => "image/jpg".to_string(),
        "gif" => "image/gif".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}