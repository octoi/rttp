mod http;
mod parser;

#[tokio::main]
async fn main() {
    let files = parser::arg::get_file_names_from_args();
    let client = reqwest::Client::new();

    for file in files {
        let data_rq = parser::file::get_data_request_from_json(file);
        http::send_request(&client, data_rq).await;
    }
}
