use std::time::Instant;

use reqwest::Client;
mod http_request;
pub mod types;

pub async fn send_request(client: &Client, data: types::DataRequest) {
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
