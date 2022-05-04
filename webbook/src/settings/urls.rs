
use std::collections::HashMap;
use crate::server::worker::{request::HttpRequest, response::HttpResponse};
use crate::settings::views;
type ViewFunction = fn(&HttpRequest) -> HttpResponse;

pub fn get_url_pattern() -> HashMap<String, ViewFunction>{
    let mut url_pattern: HashMap<String, ViewFunction> = HashMap::new();
    url_pattern.insert(String::from("/index"), views::index);
    url_pattern.insert(String::from("/now"), views::now);
    url_pattern.insert(String::from("/show_request"), views::show_request);
    url_pattern.insert(String::from("/form"), views::form);
    url_pattern.insert(String::from("/user/<user_id>/profile"), views::user_profile);
    url_pattern.insert(String::from("/login"), views::login);
    url_pattern.insert(String::from("/welcome"), views::welcome);

    return url_pattern;
}

