use serde_json::json;
use std::str::FromStr;

use crate::http::types::{DataRequest, Request};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method,
};

pub fn get_data_request_from_json(file_path: String) -> Result<DataRequest, ()> {
    // Opening file
    let file = match std::fs::File::open(file_path.clone()) {
        Ok(file) => Ok(file),
        Err(err) => {
            eprintln!("\n[-] FAILED TO OPEN FILE, {:?}\n", err.to_string());
            Err(())
        }
    }?;

    // Getting json data
    let json: serde_json::Value = match serde_json::from_reader(file) {
        Ok(json) => Ok(json),
        Err(err) => {
            eprintln!(
                "\n[-] FAILED TO READ JSON CONTENTS FROM FILE {}: {:?} \n",
                &file_path,
                err.to_string()
            );
            Err(())
        }
    }?;

    // Getting required fields
    // `url`, `method` are required fields

    let url = match json.get("url") {
        Some(url) => Ok(url.as_str().unwrap().to_string()),
        None => {
            eprintln!("\nFIELD `url` NOT FOUND IN {}\n", &file_path);
            Err(())
        }
    }?;

    let method = get_method(&json, &file_path)?;

    // Optional fields
    // `name`, `body`, `headers`, `show_error`, `show_output`, `show_status`, `show_time` are optional fields

    let name = json!(format!("{}:{}", method.to_string(), url.to_string()));
    let name = json.get("name").unwrap_or_else(|| &name).to_string(); // if name not exist, combining method:url, eg: GET:https://sample.api

    let body = match json.get("body") {
        // getting body if exist or returning an empty json
        Some(data) => data.to_owned(),
        None => json!({}),
    };

    let headers = get_headers(&json, &file_path)?;

    // Building request
    let request = Request {
        url,
        method,
        body,
        headers,
    };

    // returning DataRequest

    Ok(DataRequest::new(&name, request))
}

// get method
fn get_method(json: &serde_json::Value, file_path: &str) -> Result<Method, ()> {
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
            eprintln!("\nFIELD `method` NOT FOUND IN {}\n", &file_path);
            return Err(());
        }
    };
}

// get headers
fn get_headers(json: &serde_json::Value, file_path: &str) -> Result<HeaderMap, ()> {
    match json.get("headers") {
        Some(headers) => {
            let headers = match headers.to_owned().as_object() {
                Some(headers) => headers.to_owned(),
                None => {
                    eprintln!("Headers are not used correctly in file {}", file_path);
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
        }
        None => return Ok(HeaderMap::new()),
    }
}
