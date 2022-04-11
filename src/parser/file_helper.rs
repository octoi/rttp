use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method,
};
use serde_json::json;
use std::str::FromStr;

// Get JSON from file
// Read file & return json contents in it
pub fn read_and_get_json(file_path: &String) -> Result<serde_json::Value, ()> {
    // Opening file
    let file = match std::fs::File::open(file_path.clone()) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("\n[-] FAILED TO OPEN FILE, {:?}\n", err.to_string());
            return Err(());
        }
    };

    // Getting json data
    let json: serde_json::Value = match serde_json::from_reader(file) {
        Ok(json) => json,
        Err(err) => {
            eprintln!(
                "\n[-] FAILED TO READ JSON CONTENTS FROM FILE {}: {:?} \n",
                &file_path,
                err.to_string()
            );
            return Err(());
        }
    };

    Ok(json)
}

// Get URL
pub fn get_url(
    json: &serde_json::Value,
    file_path: &str,
    is_multiple_requests: bool,
) -> Result<String, ()> {
    match json.get("url") {
        Some(url) => Ok(url.as_str().unwrap().to_string()),
        None => {
            if !is_multiple_requests {
                eprintln!("\n[-] FIELD `url` NOT FOUND IN {}\n", &file_path);
            }
            Err(())
        }
    }
}

// Get Body
pub fn get_body(json: &serde_json::Value) -> serde_json::Value {
    match json.get("body") {
        // getting body if exist or returning an empty json
        Some(data) => data.to_owned(),
        None => json!({}),
    }
}

// get method
/*
    Reading json content and lowercasing the string to match all possible cases
        eg: "PoSt" & "POST" can be lowecased to "post", so it will work same
*/
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
