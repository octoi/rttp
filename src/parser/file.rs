use super::file_helper::{
    get_body, get_headers, get_log_option, get_method, get_url, read_and_get_json,
};
use crate::http::types::{DataRequest, Request};
use reqwest::Method;
use serde_json::json;

pub fn get_data_request_from_json(file_path: String) -> Result<Vec<DataRequest>, ()> {
    // Read & get json from file
    let json = read_and_get_json(&file_path).unwrap();

    // check if it contains multiple requests
    let requests = json.get("requests");
    let is_multiple_requests = requests.is_some();

    // get fields
    let url: Option<String> = match get_url(&json, &file_path, is_multiple_requests) {
        Ok(url) => Some(url),
        Err(_) => {
            if !is_multiple_requests {
                return Err(());
            }
            None
        }
    };

    let method: Option<Method> = match get_method(&json, &file_path) {
        Ok(method) => Some(method),
        Err(_) => {
            if !is_multiple_requests {
                return Err(());
            }
            None
        }
    };

    let body = get_body(&json);
    let headers = match get_headers(&json, &file_path) {
        Ok(headers) => Some(headers),
        Err(_) => {
            if !is_multiple_requests {
                return Err(());
            }
            None
        }
    };

    let show_error = get_log_option("show_error", &json, &file_path);
    let show_output = get_log_option("show_output", &json, &file_path);
    let show_status = get_log_option("show_status", &json, &file_path);
    let show_time = get_log_option("show_time", &json, &file_path);

    // get request if multiple requests
    let mut requests: Vec<DataRequest> = vec![];

    if !is_multiple_requests {
        let name = json!(format!(
            "{}:{}",
            // Calling unwrap because we know this file only contains on request
            method.as_ref().unwrap(),
            url.as_ref().unwrap()
        ));
        let name = json.get("name").unwrap_or_else(|| &name).to_string(); // if name not exist, combining method:url, eg: GET:https://sample.api

        // Building request
        let request = Request {
            url: url.unwrap(),
            method: method.unwrap(),
            body,
            headers: headers.unwrap(),
        };

        requests.push(DataRequest {
            name,
            request,
            show_error,
            show_output,
            show_status,
            show_time,
        })
    }

    // returning DataRequest
    Ok(requests)
}
