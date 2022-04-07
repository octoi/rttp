use super::types;
use reqwest::{Client, Method, Response};

pub async fn http_request(
    client: &Client,
    request: types::Request,
) -> Result<types::Response, types::Error> {
    let resp = client
        .request(request.method.clone(), &request.url)
        .json(&request.body)
        .headers(request.headers)
        .send()
        .await;

    handle_response(resp, request.url, request.method).await
}

async fn handle_response(
    resp: Result<Response, reqwest::Error>,
    url: String,
    method: Method,
) -> Result<types::Response, types::Error> {
    match resp {
        Err(err) => Err(types::Error::FailedToFetch(err)),
        Ok(response) => {
            let status = response.status();
            let result = response.text().await;

            match result {
                Ok(data) => Ok(types::Response::new(status, &data)),
                Err(_) => Err(types::Error::FailedToParseResponse(method, url)),
            }
        }
    }
}
