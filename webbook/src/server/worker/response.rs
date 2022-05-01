use chrono::Utc;

pub fn make_response(mut response: String, response_body:Vec<u8>, extension: &str) -> Vec<u8>{
    let mut response_head: String = Utc::now().format("Date: %a, %d %b %Y %H:%M:%S GMT\r\n").to_string();
    response_head += "Host: HenaServer/0.1\r\n";
    response_head += &format!("Content-Length: {}\r\n", response_body.len());
    response_head += "Connection: Close\r\n";
    response_head += &format!("Content-Type: {}\r\n", get_mime_type_from_extension(extension));
    response_head += "\r\n";
    
    response.push_str(&response_head);

    let mut response = response.as_bytes().to_vec();
    response.extend(response_body);
    return response;
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