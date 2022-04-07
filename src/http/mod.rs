use std::time::Instant;

use reqwest::Client;
mod http_request;
pub mod types;

#[derive(Clone)]
pub struct DataFx {
    pub name: String,
    pub request: self::types::Request,
    pub show_error: bool,
    pub show_output: bool,
    pub show_status: bool,
    pub show_time: bool,
}

impl DataFx {
    pub fn new(name: String, request: self::types::Request) -> Self {
        Self {
            name,
            request,
            show_error: true,
            show_output: true,
            show_time: true,
            show_status: true,
        }
    }
}

pub async fn send_request(client: &Client, data: DataFx) {
    println!("{}", data.name);
    let start = Instant::now();

    match http_request::http_request(client, data.request).await {
        Ok(response) => {
            if data.show_status {
                println!("{}", response.status);
            }

            if data.show_output {
                println!("{}", response.data);
            }
        }
        Err(err) => {
            if data.show_error {
                eprintln!("{}", err)
            }
        }
    }

    let duration = start.elapsed();
    if data.show_time {
        println!("completed in {:?}", duration);
    }
}
