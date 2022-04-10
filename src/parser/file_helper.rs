use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method,
};
use std::str::FromStr;

// get method
pub fn get_method(json: &serde_json::Value, file_path: &str) -> Result<Method, ()> {
    match json.get("method") {
        Some(method) => {
            return Ok(match method.as_str().unwrap().to_lowercase().as_str() {
                "get" => Method::GET,
                "post" => Method::POST,
                "put" => Method::PUT,
                "patch" => Method::PATCH,
                "delete" => Method::DELETE,
                "connect" => Method::CONNECT,
                "head" => Method::HEAD,
                "options" => Method::OPTIONS,
                "trace" => Method::TRACE,
                _ => {
                    eprintln!(
                        "\n[-] INVALID METHOD {} FOUND IN FILE{}\n",
                        method.as_str().unwrap(),
                        &file_path
                    );
                    return Err(());
                }
            })
        }
        None => {
            eprintln!("\n[-] FIELD `method` NOT FOUND IN {}\n", &file_path);
            return Err(());
        }
    };
}

// get headers
pub fn get_headers(json: &serde_json::Value, file_path: &str) -> Result<HeaderMap, ()> {
    if let Some(headers) = json.get("headers") {
        let headers = match headers.to_owned().as_object() {
            Some(headers) => headers.to_owned(),
            None => {
                eprintln!("[-] Headers are not used correctly in file {}", file_path);
                return Err(());
            }
        };
        let mut headers_map = HeaderMap::new();

        for elem in headers {
            let val: HeaderValue = elem.1.as_str().unwrap().parse().unwrap();
            let key = HeaderName::from_str(elem.0.as_str()).unwrap();
            headers_map.insert(key, val);
        }

        return Ok(headers_map);
    } else {
        return Ok(HeaderMap::new());
    }
}

// Get log options from JSON
// `show_error`, `show_time`, `show_output`, `show_status`
// Default value should be true
pub fn get_log_option(key: &str, json: &serde_json::Value, file_path: &str) -> bool {
    match json.get(key) {
        Some(val) => val.as_bool().unwrap_or_else(|| {
            eprintln!(
                "[-] FIELD `{}` IS NOT A VALID BOOLEAN (true/false) IN FILE {}",
                key, file_path
            );
            false
        }),
        None => true,
    }
}
