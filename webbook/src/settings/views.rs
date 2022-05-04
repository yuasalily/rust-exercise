use std::collections::HashMap;
use crate::settings;
use crate::server::worker::{request::HttpRequest, response::HttpResponse};
use crate::server::worker::cookie::Cookie;
use chrono::Utc;

pub fn index(request: &HttpRequest) -> HttpResponse{
    if request.get_method() == "GET"{
        let http_response = HttpResponse::new(String::from("index"));
        return http_response;
    }
    let mut http_response = HttpResponse::new(String::from(settings::NOT_ALLOWED_FILE));
    http_response.set_status_code(405);
    return http_response;
}

pub fn now(request: &HttpRequest) -> HttpResponse{
    if request.get_method() == "GET"{
        let mut http_response = HttpResponse::new(String::from("now"));
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("now"), Utc::now().format("Date: %a, %d %b %Y %H:%M:%S GMT\r\n").to_string());
        http_response.update_params(params);
        return http_response;
    }
    let mut http_response = HttpResponse::new(String::from(settings::NOT_ALLOWED_FILE));
    http_response.set_status_code(405);
    return http_response;
}

pub fn show_request(request: &HttpRequest) -> HttpResponse{
    if request.get_method() == "GET" || request.get_method() == "POST"{
        let mut http_response = HttpResponse::new(String::from("show_request"));
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("path"), request.get_path());
        params.insert(String::from("method"), request.get_method());
        params.insert(String::from("http_version"), request.get_http_version());
        let mut header: String = String::new();
        for (k, v) in request.get_header(){
            header += &format!("{}:{}\r\n",k,v);
        }
        params.insert(String::from("request_header"), header);
        let mut body: String = String::new();
        for (k, v) in request.get_body(){
            let mut q: String = String::new();
            for e in v{
                q += &e;
                q += ";";
            }
            body += &format!("{}:{}\r\n",k,q);
        }
        params.insert(String::from("request_body"),body);
        http_response.update_params(params);
        return http_response;
    }
    let mut http_response = HttpResponse::new(String::from(settings::NOT_ALLOWED_FILE));
    http_response.set_status_code(405);
    return http_response;
}

pub fn form(request: &HttpRequest) -> HttpResponse{
    if request.get_method() == "GET"{
        let http_response = HttpResponse::new(String::from("form"));
        return http_response;
    }
    let mut http_response = HttpResponse::new(String::from(settings::NOT_ALLOWED_FILE));
    http_response.set_status_code(405);
    return http_response;
}

pub fn user_profile(request: &HttpRequest) -> HttpResponse{
    if request.get_method() == "GET"{
        let mut http_response = HttpResponse::new(String::from("user_profile"));
        http_response.update_params(request.get_params());
        return http_response;
    }
    let mut http_response = HttpResponse::new(String::from(settings::NOT_ALLOWED_FILE));
    http_response.set_status_code(405);
    return http_response;
}

pub fn login(request: &HttpRequest) -> HttpResponse{
    if request.get_method() == "GET"{
        let http_response = HttpResponse::new(String::from("login"));
        return http_response;
    } else if request.get_method() == "POST"{
        let mut http_response = HttpResponse::new(String::new());
        let request_body = request.get_body();
        let username = request_body.get("username").unwrap().get(0).unwrap();
        let email = request_body.get("email").unwrap().get(0).unwrap();
        let mut header: HashMap<String, String> = HashMap::new();
        let mut cookie1 = Cookie::new(String::from("username"), String::from(username));
        cookie1.set_max_age(Some(30));
        http_response.add_cookie(cookie1);
        let mut cookie2 = Cookie::new(String::from("email"), String::from(email));
        cookie2.set_max_age(Some(30));
        http_response.add_cookie(cookie2);
        //　/welcomeにリダイレクトする
        header.insert(String::from("Location"), String::from("/welcome"));
        http_response.update_header(header);
        http_response.set_status_code(302);
        return http_response;
    }
    let mut http_response = HttpResponse::new(String::from(settings::NOT_ALLOWED_FILE));
    http_response.set_status_code(405);
    return http_response;
}

pub fn welcome(request: &HttpRequest) -> HttpResponse{
    if request.get_method() == "GET"{
        let cookies = request.get_cookies();
        let mut http_response = HttpResponse::new(String::from("welcome"));
        if !cookies.contains_key("username"){
            let mut header: HashMap<String, String> = HashMap::new();
            //　/loginにリダイレクトする
            header.insert(String::from("Location"), String::from("/login"));
            http_response.update_header(header);
            http_response.set_status_code(302);
            return http_response;
        }
        http_response.update_params(cookies);
        return http_response;
    }
    let mut http_response = HttpResponse::new(String::from(settings::NOT_ALLOWED_FILE));
    http_response.set_status_code(405);
    return http_response;
}