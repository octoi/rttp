use reqwest::Client;
use std::time::Instant;

mod http_request;
pub mod types;

pub async fn send_request(client: &Client, data: types::DataRequest) {
    println!("NAME: {}", data.name);
    println!("URL: {}", data.request.url);
    let start = Instant::now();

    match http_request::http_request(client, data.request).await {
        Ok(response) => {
            if data.show_status {
                println!("{}", response.status);
            }

            if data.show_output {
                println!("\n{}\n", response.data);
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
        println!("completed in {:?} ✨", duration);
    }
    println!("---")
}
