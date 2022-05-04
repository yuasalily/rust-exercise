use std::ffi::OsStr;
use regex::Regex;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::fs::File;
use std::env;
use std::io::Read;
use crate::settings;
use crate::settings::urls;
use crate::server::worker::{request::HttpRequest, response::HttpResponse};

pub fn mapping_url_to_view(path: &OsStr) -> (impl Fn(&HttpRequest) -> HttpResponse, HashMap<String, String>) {
    let path = path.to_str().unwrap();
    let url_pattern = urls::get_url_pattern();

    for (url, view_function) in url_pattern{
        let url = format!("^{}$",url);

        let re = Regex::new(r"<(.+?)>").unwrap();
        let url = re.replace_all(&url,r"(?P$0[^/]+)").into_owned();
    
        let pattern = Regex::new(&url).unwrap();
        if pattern.is_match(&path){
            let caps = pattern.captures(&path).unwrap();
            let pattern_map: HashMap<String, String> = pattern.capture_names().flatten().filter_map(|n| Some((n.to_string(), caps.name(n)?.as_str().to_string()))).collect();
            return (view_function, pattern_map);
        }
    }
    return (default_response, HashMap::<String, String>::new());
}

pub fn default_response(request: &HttpRequest) -> HttpResponse {
    if request.get_method() == "GET"{
        let http_response = HttpResponse::new(request.get_path());
        return http_response;
    }
    let mut http_response = HttpResponse::new(String::from(settings::NOT_FOUND_FILE));
    http_response.set_status_code(404);
    return http_response;
}


pub fn render(response:&mut HttpResponse){
    let extension = get_extension(PathBuf::from(response.get_file_name()), String::from("html"));
    let file_path = get_static_path(response.get_file_name(), &extension);
    let (response_line, response_body) = get_file(file_path, response.get_status_code());
    let response_body = if extension == "html" {
        let response_body = String::from_utf8(response_body).unwrap();
        let response_body = format_html(response_body, &response.get_params()).as_bytes().to_vec();
        response_body
    } else {response_body};
    response.set_extension(extension);
    response.set_body(response_body);
    response.set_header_line(response_line);
}

fn format_html(mut body: String, params: &HashMap<String, String>) -> String{
    //正規表現でreplaceしたら空白とか無視できそう。
    for (key, value) in params {
        body = body.replace(&("{".to_string() + key + "}"), value);
    }
    return body;
}

fn get_file(file_path: PathBuf, status_code: u32) -> (String, Vec<u8>) {
    let file = File::open(file_path);
    let (response_line, response_body) = match file {
        Ok(mut file) => {
            let mut body = Vec::new();
            let _ = file.read_to_end(&mut body);
            let line = get_status_code(status_code);
            (line, body)
        }
        Err(_err) => {
            let (line, body, _) = get_not_found();
            (line, body)
        }
    };
    return (response_line, response_body);
}

fn get_static_path(file_name: String, extension: &String) -> PathBuf{
    let mut path: PathBuf = env::current_dir().unwrap();
    let file_name_tmp = Path::new(&file_name).strip_prefix("/");
    let mut file_name = match file_name_tmp {
        Ok(file_name) => file_name.to_path_buf(),
        Err(_err) => PathBuf::from(file_name),
    };
    file_name.set_extension(extension);
    path.push(settings::STATIC_FILE_PATH);
    if extension == "html" {path.push(settings::TEMPLATES_PATH);}
    path.push(file_name);
    return path;
}

fn get_not_found() -> (String, Vec<u8>, String){
    let extension = String::from("html");
    let path = get_static_path(String::from(settings::NOT_FOUND_FILE), &extension);    
    let file = File::open(path);
    let body = match file {
        Ok(mut file) => {
            let mut body = Vec::new();
            let _ = file.read_to_end(&mut body);
            body
        }
        Err(_err) => {
            let body = "<html><body><h1>404 Not Found.</h1></body></html>".as_bytes().to_vec();
            body
        }
    };
    
    let line = get_status_code(404);
    return (line, body, extension);
}

fn get_extension(file_name: PathBuf, default: String) -> String {
    let extension = match file_name.as_path().extension(){
        Some(ex) => ex.to_str().unwrap().to_string(),
        None => default,
    };
    return extension
}

fn get_status_code(code: u32) -> String {
    match code {
        200 => "HTTP/1.1 200 OK\r\n".to_string(),
        302 => "HTTP/1.1 302 Found\r\n".to_string(),
        404 => "HTTP/1.1 404 Not Found\r\n".to_string(),
        405 => "HTTP/1.1 405 Not Allowed\r\n".to_string(),
        _ => "HTTP/1.1 404 Not Found\r\n".to_string(),
    }
}