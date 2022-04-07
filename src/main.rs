use http::{
    send_request,
    types::{DataRequest, Request},
};
use reqwest::{header::HeaderMap, Method};
use serde_json::json;

mod http;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let rq = Request {
        body: json!({}),
        headers: HeaderMap::new(),
        url: "https://jsonplaceholder.typicode.com/todos/1".to_string(),
        method: Method::GET,
    };

    let data = DataRequest::new("Geting /todos/1", rq);

    send_request(&client, data.clone()).await;
    send_request(&client, data.clone()).await;
    send_request(&client, data.clone()).await;
    send_request(&client, data.clone()).await;
    send_request(&client, data.clone()).await;
}
