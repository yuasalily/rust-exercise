use std::ffi::OsStr;
use std::collections::HashMap;
use crate::server::worker::view;

pub fn mapping_url_to_view(path: &OsStr) -> impl Fn(&HashMap<String, String>) -> (String, Vec<u8>, String) {
    let path = path.to_str().unwrap();
    match path{
        "index" => view::index,
        "now" => view::now,
        "show_request" => view::show_request,
        "form" => view::form,
        _ => view::default_response,
    }
}