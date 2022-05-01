use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::File;
use std::env;
use std::io::Read;

pub fn index(request: &HashMap<String, String>) -> (String, Vec<u8>, String){
    if request.get("method").unwrap() == "GET"{
        let extension = get_extension(PathBuf::from(request.get("path").unwrap()), String::from("html"));
        return get_text_response(request, extension);
    }
    let (response_line, response_body, extension) = get_not_allowed();
    return (response_line, response_body, extension);
}

pub fn now(request: &HashMap<String, String>) -> (String, Vec<u8>, String) {
    if request.get("method").unwrap() == "GET"{
        let extension = get_extension(PathBuf::from(request.get("path").unwrap()), String::from("html"));
        return get_text_response(request, extension);
    }
    let (response_line, response_body, extension) = get_not_allowed();
    return (response_line, response_body, extension);
}

pub fn show_request(request: &HashMap<String, String>) -> (String, Vec<u8>, String) {
    if request.get("method").unwrap() == "POST"{
        let extension = get_extension(PathBuf::from(request.get("path").unwrap()), String::from("html"));
        return get_text_response(request, extension);
    }
    let (response_line, response_body, extension) = get_not_allowed();
    return (response_line, response_body, extension);
}

pub fn form(request: &HashMap<String, String>) -> (String, Vec<u8>, String) {
    if request.get("method").unwrap() == "GET"{
        let extension = get_extension(PathBuf::from(request.get("path").unwrap()), String::from("html"));
        return get_text_response(request, extension);
    }
    let (response_line, response_body, extension) = get_not_allowed();
    return (response_line, response_body, extension);
}

pub fn default_response(request: &HashMap<String, String>) -> (String, Vec<u8>, String) {
    if request.get("method").unwrap() == "GET"{
        let extension = get_extension(PathBuf::from(request.get("path").unwrap()), String::from("htnl"));
        return get_response(request, extension);
    }
    let (response_line, response_body, extension) = get_not_allowed();
    return (response_line, response_body, extension);
}

fn get_text_response(request: &HashMap<String, String>, extension: String) -> (String, Vec<u8>, String) {
    let file_path = get_static_path(request.get("path").unwrap().clone(), &extension);
    let (response_line, response_body) = get_file(file_path);
    let response_body = String::from_utf8(response_body).unwrap();
    let response_body = format_html(response_body, request).as_bytes().to_vec();
    return (response_line, response_body, extension);
}

fn get_response(request: &HashMap<String, String>, extension: String) -> (String, Vec<u8>, String) {
    let file_path = get_static_path(request.get("path").unwrap().clone(), &extension);
    let (response_line, response_body) = get_file(file_path);
    return (response_line, response_body, extension);
}

fn format_html(mut body: String, params: &HashMap<String, String>) -> String{
    for (key, value) in params {
        body = body.replace(&("{".to_string() + key + "}"), value);
    }
    return body;
}

fn get_file(file_path: PathBuf) -> (String, Vec<u8>) {
    let file = File::open(file_path);
    let (response_line, response_body) = match file {
        Ok(mut file) => {
            let mut body = Vec::new();
            let _ = file.read_to_end(&mut body);
            let line = "HTTP/1.1 200 OK\r\n".to_string();
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
    let mut file_name = PathBuf::from(PathBuf::from(file_name).file_stem().unwrap());
    file_name.set_extension(extension);
    path.push("static");
    path.push(file_name);
    return path;
}

fn get_not_found() -> (String, Vec<u8>, String){
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push("static");
    path.push("404.html");
    let mut body = Vec::new();
    let mut file = File::open(path).unwrap();
    let _ = file.read_to_end(&mut body);
    let line = "HTTP/1.1 404 Not Found\r\n".to_string();
    return (line, body, String::from("html"));
}

fn get_not_allowed() -> (String, Vec<u8>, String){
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push("static");
    path.push("405.html");
    let mut body = Vec::new();
    let mut file = File::open(path).unwrap();
    let _ = file.read_to_end(&mut body);
    let line = "HTTP/1.1 405 Not Allowed\r\n".to_string();
    return (line, body, String::from("html"));
}

fn get_extension(file_name: PathBuf, default: String) -> String {
    let extension = match file_name.as_path().extension(){
        Some(ex) => ex.to_str().unwrap().to_string(),
        None => default,
    };
    return extension
}