use super::file_helper::{get_headers, get_log_option, get_method};
use crate::http::types::{DataRequest, Request};
use serde_json::json;

pub fn get_data_request_from_json(file_path: String) -> Result<Vec<DataRequest>, ()> {
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

    // TODO: check whether the file contains multiple requests

    // Getting required fields
    // `url`, `method` are required fields

    let url = match json.get("url") {
        Some(url) => Ok(url.as_str().unwrap().to_string()),
        None => {
            eprintln!("\n[-] FIELD `url` NOT FOUND IN {}\n", &file_path);
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

    // Log options
    let show_error = get_log_option("show_error", &json, &file_path);
    let show_output = get_log_option("show_output", &json, &file_path);
    let show_status = get_log_option("show_status", &json, &file_path);
    let show_time = get_log_option("show_time", &json, &file_path);

    // Building request
    let request = Request {
        url,
        method,
        body,
        headers,
    };

    // returning DataRequest
    Ok(vec![DataRequest {
        name,
        request,
        show_error,
        show_output,
        show_status,
        show_time,
    }])
}
