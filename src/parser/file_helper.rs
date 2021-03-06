use colored::Colorize;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method,
};
use serde_json::json;
use std::str::FromStr;

use crate::http::types::{DataRequest, Request};

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
pub fn get_url(json: &serde_json::Value, file_path: &str) -> Result<String, String> {
    match json.get("url") {
        Some(url) => Ok(url.as_str().unwrap().to_string()),
        None => Err(format!("[-] FIELD `url` NOT FOUND IN {}", &file_path)),
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
pub fn get_method(json: &serde_json::Value, file_path: &str) -> Result<Method, String> {
    match json.get("method") {
        Some(method) => {
            let method = match method.as_str().unwrap().to_lowercase().as_str() {
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
                    return Err(format!(
                        "[-] INVALID METHOD {} FOUND IN FILE{}",
                        method.as_str().unwrap(),
                        &file_path
                    ));
                }
            };

            Ok(method)
        }
        None => Err(format!("[-] FIELD `method` NOT FOUND IN {}", &file_path)),
    }
}

// get headers
pub fn get_headers(json: &serde_json::Value, file_path: &str) -> Result<HeaderMap, String> {
    if let Some(headers) = json.get("headers") {
        let headers = match headers.to_owned().as_object() {
            Some(headers) => headers.to_owned(),
            None => {
                return Err(format!(
                    "[-] Headers are not used correctly in file {}",
                    file_path
                ));
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
pub fn get_log_option(
    key: &str,
    json: &serde_json::Value,
    file_path: &str,
    default: Option<bool>,
) -> Result<bool, String> {
    match json.get(key) {
        Some(val) => match val.as_bool() {
            Some(val) => Ok(val),
            None => Err(format!(
                "[-] FIELD `{}` IS NOT A VALID BOOLEAN (true/false) IN FILE {}",
                key, file_path
            )),
        },
        None => Ok(default.unwrap_or(true)),
    }
}

// Helper function to build a request from a single file
pub fn craft_data_request(
    json: &serde_json::Value,
    url: String,
    method: Method,
    body: serde_json::Value,
    headers: HeaderMap,
    show_error: bool,
    show_output: bool,
    show_status: bool,
    show_time: bool,
) -> DataRequest {
    let name = match json.get("name") {
        Some(name) => format!(
            "{}",
            name.as_str()
                .unwrap_or(name.to_string().as_str())
                .to_string()
                .cyan()
        ),
        None => format!(
            "{}{}{}",
            method.to_string().yellow(),
            ":".white(),
            url.cyan()
        ),
    };

    // Building request
    let request = Request {
        url,
        method,
        body,
        headers,
    };

    DataRequest {
        name,
        request,
        show_error,
        show_output,
        show_status,
        show_time,
    }
}

// Validate global fields
pub fn validate_field<T, E: std::fmt::Display>(
    result: Result<T, E>,
    is_multiple_requests: bool,
) -> Result<Option<T>, ()> {
    match result {
        Ok(val) => Ok(Some(val)),
        Err(err) => {
            if !is_multiple_requests {
                eprintln!("\n{}\n", err);
                return Err(());
            }
            Ok(None)
        }
    }
}

// Use global fields in requests
pub fn use_global_field<T, E: std::fmt::Display>(
    result: Result<T, E>,
    swap: Option<T>,
) -> Result<T, ()> {
    match result {
        Ok(val) => Ok(val),
        Err(err) => {
            if swap.is_some() {
                Ok(swap.unwrap())
            } else {
                eprintln!("\n{}\n", err);
                return Err(());
            }
        }
    }
}

// Convert Result<T, String> to Result<T, ()>
pub fn handle_result_error<T>(result: Result<T, String>) -> Result<T, ()> {
    match result {
        Ok(val) => Ok(val),
        Err(err) => {
            eprintln!("{}", err);
            Err(())
        }
    }
}
