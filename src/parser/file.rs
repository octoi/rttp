use crate::http::types::{DataRequest, Request};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method,
};
use serde_json::json;
use std::str::FromStr;

pub fn get_data_request_from_json(filename: String) -> DataRequest {
    // Opening & getting json data
    let file = std::fs::File::open(filename).expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    // Required fields
    let url = json
        .get("url")
        .expect("URL not found")
        .as_str()
        .unwrap()
        .to_string();

    let method = json
        .get("method")
        .expect("METHOD not found")
        .as_str()
        .unwrap()
        .to_lowercase();

    let method = match method.as_str() {
        "get" => Method::GET,
        "post" => Method::POST,
        "put" => Method::PUT,
        "patch" => Method::PATCH,
        "delete" => Method::DELETE,
        "connect" => Method::CONNECT,
        "head" => Method::HEAD,
        "options" => Method::OPTIONS,
        "trace" => Method::TRACE,
        _ => Method::GET,
    };

    // Optional fields
    let name = json!(format!("{}:{}", method.to_string(), url.to_string()));
    let name = json.get("name").unwrap_or_else(|| &name).to_string(); // if name not exist, combining method:url, eg: GET:https://sample.api

    let body = match json.get("body") {
        // getting body if exist or returning an empty json
        Some(data) => data.to_owned(),
        None => json!({}),
    };

    let headers: HeaderMap = match json.get("headers") {
        Some(headers) => {
            let headers = headers.to_owned().as_object().unwrap().to_owned();
            let mut headers_map = HeaderMap::new();

            for elem in headers {
                let val: HeaderValue = elem.1.as_str().unwrap().parse().unwrap();
                let key = HeaderName::from_str(elem.0.as_str()).unwrap();
                headers_map.insert(key, val);
            }

            headers_map
        }
        None => HeaderMap::new(),
    };

    let request = Request {
        url,
        method,
        body,
        headers,
    };

    DataRequest::new(&name, request)
}
