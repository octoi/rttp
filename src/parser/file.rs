use super::file_helper::{
    craft_data_request, get_body, get_headers, get_log_option, get_method, get_url,
    handle_result_error, read_and_get_json, use_global_field, validate_field,
};
use crate::http::types::DataRequest;
use reqwest::header::HeaderMap;

pub fn get_data_request_from_json(file_path: String) -> Result<Vec<DataRequest>, ()> {
    // Read & get json from file
    let json = read_and_get_json(&file_path).unwrap();

    // check if it contains multiple requests
    let requests = json.get("requests");
    let is_multiple_requests = requests.is_some();

    // get fields
    let url = validate_field(get_url(&json, &file_path), is_multiple_requests)?;

    let method = validate_field(get_method(&json, &file_path), is_multiple_requests)?;

    let body = get_body(&json);
    let headers = validate_field(get_headers(&json, &file_path), is_multiple_requests)?;

    let show_error = validate_field(
        get_log_option("show_error", &json, &file_path, None),
        is_multiple_requests,
    )?;
    let show_output = validate_field(
        get_log_option("show_output", &json, &file_path, None),
        is_multiple_requests,
    )?;
    let show_status = validate_field(
        get_log_option("show_status", &json, &file_path, None),
        is_multiple_requests,
    )?;
    let show_time = validate_field(
        get_log_option("show_time", &json, &file_path, None),
        is_multiple_requests,
    )?;

    // get request if multiple requests
    let mut data_requests: Vec<DataRequest> = vec![];

    if !is_multiple_requests {
        let request = craft_data_request(
            &json,
            url.unwrap(),
            method.unwrap(),
            body,
            headers.unwrap(),
            show_error.unwrap(),
            show_output.unwrap(),
            show_status.unwrap(),
            show_time.unwrap(),
        );
        data_requests.push(request)
    } else {
        let requests = match requests.unwrap().as_array() {
            Some(requests) => requests.to_owned(),
            None => {
                eprintln!("[-] FIELD `requests` IS NOT A VALID ARRAY");
                return Err(());
            }
        };

        for request in requests {
            let request_url = use_global_field(get_url(&request, &file_path), url.clone())?;

            let request_method =
                use_global_field(get_method(&request, &file_path), method.clone())?;

            let request_body = get_body(&request);

            let mut request_headers = HeaderMap::new();
            let headers_in_request = handle_result_error(get_headers(&request, &file_path))?;

            if headers.clone().is_some() {
                request_headers = headers.clone().unwrap();

                for (k, v) in headers_in_request.iter() {
                    request_headers.append(k.to_owned(), v.to_owned());
                }
            }

            let request_show_error = handle_result_error(get_log_option(
                "show_error",
                &request,
                &file_path,
                show_error,
            ))?;
            let request_show_output = handle_result_error(get_log_option(
                "show_output",
                &request,
                &file_path,
                show_output,
            ))?;
            let request_show_status = handle_result_error(get_log_option(
                "show_status",
                &request,
                &file_path,
                show_status,
            ))?;
            let request_show_time =
                handle_result_error(get_log_option("show_time", &request, &file_path, show_time))?;

            let data_request = craft_data_request(
                &json,
                request_url,
                request_method,
                request_body,
                request_headers,
                request_show_error,
                request_show_output,
                request_show_status,
                request_show_time,
            );

            data_requests.push(data_request);
        }
    }

    // returning DataRequest
    Ok(data_requests)
}
