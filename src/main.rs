mod http;
mod parser;

#[tokio::main]
async fn main() {
    let files = parser::arg::get_file_names_from_args();
    let client = reqwest::Client::new();

    for file in files {
        let data_rqs = parser::file::get_data_request_from_json(file);

        if data_rqs.is_ok() {
            // using `.unwrap()` because we know it will never fail
            let data_rqs = data_rqs.unwrap();

            for data_rq in data_rqs {
                http::send_request(&client, data_rq).await;
            }
        }
    }
}
